//! O0.5 — SQLite WAL events, atomic outbox + projection + receipt, append-only
//! hash-chained audit, forced sink redaction, and vault refs (ADR-005).
//! A no-op still produces a receipt and an audit row.

pub mod redaction;
pub mod vault;

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use optimal_common::{now_rfc3339, CorrelationId, Receipt};
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::Value;
use sha2::{Digest, Sha256};

use redaction::RedactingSink;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("audit chain invalid: {0}")]
    Chain(String),
    #[error("internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, StoreError>;

pub struct EventStore {
    conn: Mutex<Connection>,
    transcript: PathBuf,
    sink: RedactingSink,
}

impl EventStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=FULL;
             PRAGMA foreign_keys=ON;
             CREATE TABLE IF NOT EXISTS events (
               seq INTEGER PRIMARY KEY AUTOINCREMENT,
               stream_id TEXT NOT NULL,
               type TEXT NOT NULL,
               payload TEXT NOT NULL,
               causation_id TEXT,
               correlation_id TEXT NOT NULL,
               ts TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS projection (
               stream_id TEXT PRIMARY KEY,
               last_seq INTEGER NOT NULL,
               checksum TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS outbox (
               id INTEGER PRIMARY KEY AUTOINCREMENT,
               event_seq INTEGER NOT NULL,
               payload TEXT NOT NULL,
               delivered INTEGER NOT NULL DEFAULT 0
             );
             CREATE TABLE IF NOT EXISTS receipts (
               id TEXT PRIMARY KEY,
               event_seq INTEGER NOT NULL,
               tool_id TEXT NOT NULL,
               correlation_id TEXT NOT NULL,
               tier TEXT NOT NULL,
               rule_id TEXT,
               scope TEXT NOT NULL,
               effect_hash TEXT NOT NULL,
               no_op INTEGER NOT NULL,
               ts TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS audit (
               row_id INTEGER PRIMARY KEY AUTOINCREMENT,
               event_seq INTEGER NOT NULL,
               prev_hash TEXT NOT NULL,
               hash TEXT NOT NULL,
               ts TEXT NOT NULL
             );",
        )?;
        let transcript = path.with_extension("jsonl");
        Ok(Self { conn: Mutex::new(conn), transcript, sink: RedactingSink::default() })
    }

    pub fn sink(&self) -> &RedactingSink { &self.sink }

    /// Commit event + projection + outbox + receipt + audit row atomically.
    /// `payload` is force-redacted before any of it is written.
    pub fn append_event(
        &self,
        stream_id: &str,
        event_type: &str,
        payload: &Value,
        causation_id: Option<&str>,
        receipt: &Receipt,
    ) -> Result<u64> {
        let redacted = self.sink.redact_json(payload);
        let payload_json = serde_json::to_string(&redacted).unwrap();
        let ts = now_rfc3339();
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO events (stream_id,type,payload,causation_id,correlation_id,ts)
             VALUES (?1,?2,?3,?4,?5,?6)",
            params![stream_id, event_type, payload_json, causation_id, receipt.correlation_id.as_str(), ts],
        )?;
        let seq = tx.last_insert_rowid() as u64;

        let prev_checksum: String = tx
            .query_row("SELECT checksum FROM projection WHERE stream_id=?1", params![stream_id], |r| r.get(0))
            .optional()?
            .unwrap_or_else(|| "0".repeat(64));
        let checksum = chain_hash(&prev_checksum, &format!("{stream_id}|{seq}|{event_type}|{payload_json}"));
        tx.execute(
            "INSERT INTO projection (stream_id,last_seq,checksum) VALUES (?1,?2,?3)
             ON CONFLICT(stream_id) DO UPDATE SET last_seq=excluded.last_seq, checksum=excluded.checksum",
            params![stream_id, seq, checksum],
        )?;

        tx.execute(
            "INSERT INTO outbox (event_seq,payload) VALUES (?1,?2)",
            params![seq, payload_json],
        )?;
        tx.execute(
            "INSERT INTO receipts (id,event_seq,tool_id,correlation_id,tier,rule_id,scope,effect_hash,no_op,ts)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            params![
                receipt.id, seq, receipt.tool_id, receipt.correlation_id.as_str(),
                receipt.tier, receipt.rule_id, receipt.scope, receipt.effect_hash,
                receipt.no_op as i32, receipt.ts,
            ],
        )?;

        let prev_hash: String = tx
            .query_row("SELECT hash FROM audit ORDER BY row_id DESC LIMIT 1", [], |r| r.get(0))
            .optional()?
            .unwrap_or_else(|| "0".repeat(64));
        let hash = chain_hash(&prev_hash, &format!("{seq}|{event_type}|{payload_json}"));
        tx.execute(
            "INSERT INTO audit (event_seq,prev_hash,hash,ts) VALUES (?1,?2,?3,?4)",
            params![seq, prev_hash, hash, ts],
        )?;
        tx.commit()?;

        // JSONL transcript mirror (not authoritative; the SQLite row is).
        let line = serde_json::json!({
            "seq": seq,
            "stream_id": stream_id,
            "type": event_type,
            "payload": redacted,
            "causation_id": causation_id,
            "correlation_id": receipt.correlation_id.as_str(),
            "ts": ts,
        });
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&self.transcript)?;
        writeln!(f, "{}", serde_json::to_string(&line).unwrap())?;
        Ok(seq)
    }

    /// Recompute the entire audit hash chain. Returns Ok(false) on any break.
    pub fn verify_chain(&self) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT event_seq,prev_hash,hash FROM audit ORDER BY row_id")?;
        let rows = stmt.query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?)))?;
        let mut prev = "0".repeat(64);
        for row in rows {
            let (seq, stored_prev, stored_hash) = row?;
            if stored_prev != prev {
                return Ok(false);
            }
            // Payload is no longer needed: the audit row stores the hash of
            // seq|type|payload, so verification re-derives from the event row.
            let (etype, payload): (String, String) = conn.query_row(
                "SELECT type,payload FROM events WHERE seq=?1", params![seq], |r| Ok((r.get(0)?, r.get(1)?)),
            )?;
            let expected = chain_hash(&prev, &format!("{seq}|{etype}|{payload}"));
            if expected != stored_hash {
                return Ok(false);
            }
            prev = stored_hash;
        }
        Ok(true)
    }

    pub fn receipt_count(&self) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        Ok(conn.query_row("SELECT COUNT(*) FROM receipts", [], |r| r.get(0))?)
    }

    pub fn pending_outbox(&self) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        Ok(conn.query_row("SELECT COUNT(*) FROM outbox WHERE delivered=0", [], |r| r.get(0))?)
    }
}

pub fn chain_hash(prev: &str, body: &str) -> String {
    let mut h = Sha256::new();
    h.update(prev.as_bytes());
    h.update(b"|");
    h.update(body.as_bytes());
    hex::encode(h.finalize())
}

/// Convenience: build a no-op receipt for a tool call (still audited).
pub fn no_op_receipt(tool_id: &str, tier: &str, correlation_id: &CorrelationId) -> Receipt {
    let mut r = Receipt::new(tool_id, tier, correlation_id);
    r.no_op = true;
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn event(store: &EventStore, secret: bool) -> u64 {
        let payload = if secret {
            json!({"text": "API_KEY=canary-O0-SECRET-9c2b"})
        } else {
            json!({"text": "read-only no-op"})
        };
        let r = Receipt::new("fs.read", "readonly", &CorrelationId::from_string("test-corr"));
        store.append_event("task-1", "tool.result", &payload, None, &r).unwrap()
    }

    #[test]
    fn wal_mode_is_enabled() {
        let dir = tempfile::tempdir().unwrap();
        let store = EventStore::open(dir.path().join("events.db")).unwrap();
        let mode: String = store.conn.lock().unwrap().query_row("PRAGMA journal_mode", [], |r| r.get(0)).unwrap();
        assert_eq!(mode.to_ascii_lowercase(), "wal");
    }

    #[test]
    fn atomic_append_writes_event_projection_outbox_receipt_and_audit() {
        let dir = tempfile::tempdir().unwrap();
        let store = EventStore::open(dir.path().join("events.db")).unwrap();
        let seq = event(&store, false);
        assert_eq!(seq, 1);
        assert_eq!(store.receipt_count().unwrap(), 1);
        assert_eq!(store.pending_outbox().unwrap(), 1);
        assert!(store.verify_chain().unwrap());
    }

    #[test]
    fn no_op_still_gets_receipt_and_audit_row() {
        let dir = tempfile::tempdir().unwrap();
        let store = EventStore::open(dir.path().join("events.db")).unwrap();
        let r = no_op_receipt("tools.search", "readonly", &CorrelationId::new());
        assert!(r.no_op);
        store.append_event("task-1", "tool.result", &json!({}), None, &r).unwrap();
        assert_eq!(store.receipt_count().unwrap(), 1);
    }

    #[test]
    fn secret_never_reaches_events_jsonl() {
        let dir = tempfile::tempdir().unwrap();
        let store = EventStore::open(dir.path().join("events.db")).unwrap();
        event(&store, true);
        let transcript = std::fs::read_to_string(dir.path().join("events.jsonl")).unwrap();
        assert!(!transcript.contains("canary-O0-SECRET-9c2b"));
        assert!(transcript.contains("[REDACTED]"));
    }

    #[test]
    fn tampering_breaks_the_chain() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("events.db");
        let store = EventStore::open(&db).unwrap();
        event(&store, false);
        event(&store, false);
        assert!(store.verify_chain().unwrap());
        drop(store);
        {
            let conn = Connection::open(&db).unwrap();
            conn.execute("UPDATE events SET payload='tampered' WHERE seq=1", []).unwrap();
        }
        let store = EventStore::open(&db).unwrap();
        assert!(!store.verify_chain().unwrap(), "tampered chain must fail verification");
    }
}
