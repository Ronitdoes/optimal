//! Shared, dependency-light types for the Optimal O0 skeleton.
//!
//! Every governed path in the system carries a `CorrelationId`, returns a
//! uniform `ToolResult`, and (for any effect, including no-ops) produces a
//! `Receipt`. Nothing in this crate touches the OS.

use std::fmt;

use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Stable, end-to-end correlation handle. Propagated core -> tool -> effect ->
/// audit row -> gateway stream. Mirrors the W3C `traceparent` mapping in ADR-010.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CorrelationId(String);

impl CorrelationId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CorrelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Uniform result envelope. `exit` is the raw child exit code (0 on denied /
/// synthetic results), kept separate from `status` so a rendered card can show
/// both the policy decision and the process outcome.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolResult {
    pub status: Status,
    pub stdout: String,
    pub stderr: String,
    pub exit: i32,
    pub artifacts: Vec<Artifact>,
    pub duration_ms: u64,
    pub correlation_id: CorrelationId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ok,
    Error,
    Denied,
    Timeout,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Artifact {
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
}

impl ToolResult {
    pub fn ok(stdout: impl Into<String>, correlation_id: &CorrelationId) -> Self {
        Self {
            status: Status::Ok,
            stdout: stdout.into(),
            stderr: String::new(),
            exit: 0,
            artifacts: Vec::new(),
            duration_ms: 0,
            correlation_id: correlation_id.clone(),
        }
    }

    /// A denied call is still a first-class result: it carries the rule that
    /// denied it in `stderr` so a model cannot be told "denied" without the
    /// explicit anti-retry instruction (ADR-014 / O1.3 vocabulary).
    pub fn denied(reason: impl Into<String>, correlation_id: &CorrelationId) -> Self {
        Self {
            status: Status::Denied,
            stdout: String::new(),
            stderr: format!("DENIED: {}. Do not retry or route around this decision.", reason.into()),
            exit: 0,
            artifacts: Vec::new(),
            duration_ms: 0,
            correlation_id: correlation_id.clone(),
        }
    }

    pub fn error(stderr: impl Into<String>, correlation_id: &CorrelationId) -> Self {
        Self {
            status: Status::Error,
            stdout: String::new(),
            stderr: stderr.into(),
            exit: 1,
            artifacts: Vec::new(),
            duration_ms: 0,
            correlation_id: correlation_id.clone(),
        }
    }

    pub fn timeout(correlation_id: &CorrelationId) -> Self {
        Self {
            status: Status::Timeout,
            stdout: String::new(),
            stderr: "timeout".into(),
            exit: 124,
            artifacts: Vec::new(),
            duration_ms: 0,
            correlation_id: correlation_id.clone(),
        }
    }

    pub fn with_duration_ms(mut self, ms: u64) -> Self {
        self.duration_ms = ms;
        self
    }

    pub fn with_artifact(mut self, a: Artifact) -> Self {
        self.artifacts.push(a);
        self
    }
}

/// Proof that an effect (or deliberate no-op) was governed. Persisted by the
/// store crate into both the event row and the hash-chained audit row.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt {
    pub id: String,
    pub tool_id: String,
    pub correlation_id: CorrelationId,
    pub tier: String,
    pub rule_id: Option<String>,
    pub scope: String,
    pub effect_hash: String,
    pub ts: String,
    pub no_op: bool,
}

impl Receipt {
    pub fn new(
        tool_id: impl Into<String>,
        tier: impl Into<String>,
        correlation_id: &CorrelationId,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            tool_id: tool_id.into(),
            correlation_id: correlation_id.clone(),
            tier: tier.into(),
            rule_id: None,
            scope: String::new(),
            effect_hash: String::new(),
            ts: now_rfc3339(),
            no_op: true,
        }
    }
}

pub fn now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("policy denied: {0}")]
    Denied(String),
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("tool unavailable: {0}")]
    Unavailable(String),
    #[error("store error: {0}")]
    Store(String),
    #[error("transport error: {0}")]
    Transport(String),
    #[error("internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn denied_result_has_anti_retry_instruction() {
        let r = ToolResult::denied("never", &CorrelationId::new());
        assert_eq!(r.status, Status::Denied);
        assert!(r.stderr.contains("Do not retry or route around"));
    }

    #[test]
    fn correlation_ids_are_unique_and_wrap_json() {
        let a = CorrelationId::new();
        let b = CorrelationId::new();
        assert_ne!(a, b);
        let j = serde_json::to_string(&a).unwrap();
        assert_eq!(serde_json::from_str::<CorrelationId>(&j).unwrap(), a);
    }
}
