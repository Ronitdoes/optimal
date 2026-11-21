use anyhow::{Result, anyhow, ensure};
use optimal::protocol::{Command, DEMO_STEPS, Event, EventKind, connect};
use rusqlite::{Connection, Error, ErrorCode, params};
use std::{path::Path, time::Duration};
use tokio::sync::mpsc;

pub async fn run(root: &Path) -> Result<()> {
    let scratch = tempfile::tempdir_in(root)?;
    let (commands, events) = connect(&tokio::runtime::Handle::current());
    commands
        .send(Command::start_demo(1))
        .map_err(|_| anyhow!("the Tokio command receiver closed"))?;
    let path = scratch.path().join("events.sqlite");
    tokio::task::spawn_blocking(move || persist_and_reopen(&path, events)).await??;
    println!(
        "PASS: SQLite committed and replayed ordered events after reopening and rejected a duplicate sequence."
    );
    Ok(())
}

fn persist_and_reopen(path: &Path, mut events: mpsc::Receiver<Event>) -> Result<()> {
    let mut db = Connection::open(path)?;
    db.busy_timeout(Duration::from_secs(2))?;
    let mode: String = db.query_row("PRAGMA journal_mode = WAL", [], |row| row.get(0))?;
    ensure!(mode == "wal", "SQLite did not enable WAL");
    db.execute_batch(
        "PRAGMA foreign_keys = ON;
         CREATE TABLE probe_events (
             run_id INTEGER NOT NULL,
             sequence INTEGER NOT NULL,
             payload TEXT NOT NULL,
             PRIMARY KEY (run_id, sequence)
         );",
    )?;

    let mut observed = Vec::new();
    while let Some(event) = events.blocking_recv() {
        let tx = db.transaction()?;
        tx.execute(
            "INSERT INTO probe_events (run_id, sequence, payload) VALUES (?1, ?2, ?3)",
            params![event.run_id, event.sequence, serde_json::to_string(&event)?],
        )?;
        tx.commit()?;
        observed.push(event);
    }
    ensure!(
        observed.len() == usize::from(DEMO_STEPS) + 2,
        "wrong event count"
    );
    ensure!(observed[0].kind == EventKind::Started, "missing start");
    for (index, event) in observed.iter().enumerate() {
        ensure!(event.sequence == index as u64 + 1, "out-of-order event");
    }
    ensure!(
        observed.last().unwrap().kind == EventKind::Completed,
        "missing completion"
    );
    let duplicate = db.execute("INSERT INTO probe_events VALUES (1, 1, '')", []);
    ensure!(
        matches!(duplicate, Err(Error::SqliteFailure(error, _)) if error.code == ErrorCode::ConstraintViolation),
        "SQLite failed to reject a duplicate sequence"
    );
    drop(db);

    let reopened = Connection::open(path)?;
    let mut select =
        reopened.prepare("SELECT payload FROM probe_events WHERE run_id = 1 ORDER BY sequence")?;
    let replayed = select
        .query_map([], |row| row.get::<_, String>(0))?
        .map(|row| Ok(serde_json::from_str::<Event>(&row?)?))
        .collect::<Result<Vec<_>>>()?;
    ensure!(
        observed == replayed,
        "reopened database differs from the event stream"
    );
    Ok(())
}
