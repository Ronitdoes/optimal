//! O0.3 — Native agent loop (ADR-023: no LangGraph / graph framework).
//! PERCEIVE -> PLAN -> ACT -> REFLECT as an owned state machine over the plan
//! file. Interrupts are checked between tools; the kill switch is a hard stop
//! that still leaves a receipt/effect log for everything already completed.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use optimal_common::{now_rfc3339, CorrelationId, Status, ToolResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// ---------------------------------------------------------------------------
// Plan-as-file
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepStatus {
    #[default]
    Pending,
    Running,
    Done,
    Failed,
    Skipped,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Step {
    pub id: String,
    pub title: String,
    pub tool: String,
    pub command: String,
    #[serde(default)]
    pub status: StepStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plan {
    pub goal: String,
    pub steps: Vec<Step>,
    #[serde(default)]
    pub updated_at: String,
}

/// Durable `plan.json` + human-readable `todos.md`, written atomically
/// (temp file + rename) so a crash cannot leave a half-written plan.
#[derive(Clone, Debug)]
pub struct PlanStore {
    dir: PathBuf,
}

impl PlanStore {
    pub fn new(dir: impl AsRef<Path>) -> Self {
        Self { dir: dir.as_ref().to_path_buf() }
    }

    pub fn plan_path(&self) -> PathBuf { self.dir.join("plan.json") }
    pub fn todos_path(&self) -> PathBuf { self.dir.join("todos.md") }

    pub fn save(&self, plan: &Plan) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.dir)?;
        let mut p = plan.clone();
        p.updated_at = now_rfc3339();
        atomic_write(&self.plan_path(), serde_json::to_string_pretty(&p).unwrap().as_bytes())?;
        let mut todos = String::from("# Optimal task todos\n\n");
        for s in &p.steps {
            todos.push_str(&format!("- [{}] {} — {}\n", mark(&s.status), s.title, status_str(&s.status)));
        }
        todos.push_str(&format!("\nGoal: {}\nUpdated: {}\n", p.goal, p.updated_at));
        atomic_write(&self.todos_path(), todos.as_bytes())
    }

    pub fn load(&self) -> std::io::Result<Option<Plan>> {
        match std::fs::read_to_string(self.plan_path()) {
            Ok(s) => Ok(serde_json::from_str(&s).ok()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }
}

fn mark(s: &StepStatus) -> &'static str {
    match s {
        StepStatus::Done => "x",
        _ => " ",
    }
}
fn status_str(s: &StepStatus) -> &'static str {
    match s {
        StepStatus::Pending => "pending",
        StepStatus::Running => "running",
        StepStatus::Done => "done",
        StepStatus::Failed => "failed",
        StepStatus::Skipped => "skipped",
    }
}

fn atomic_write(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, bytes)?;
    std::fs::rename(&tmp, path)
}

/// Stable idempotency key for a step attempt: identical inputs never produce a
/// duplicate effect key across retries.
pub fn idempotency_key(step_id: &str, command: &str, correlation_id: &CorrelationId) -> String {
    let mut h = Sha256::new();
    h.update(step_id.as_bytes());
    h.update(b":");
    h.update(command.as_bytes());
    h.update(b":");
    h.update(correlation_id.as_str().as_bytes());
    hex_encode(&h.finalize())
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0xf) as usize] as char);
    }
    s
}

// ---------------------------------------------------------------------------
// Interrupts + kill switch
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    /// Esc — soft steering injected between tools.
    Soft(String),
    /// Ctrl+Alt+Space — hard kill of in-flight work.
    Kill,
}

#[derive(Clone, Debug)]
pub struct InterruptQueue {
    tx: tokio::sync::mpsc::Sender<Interrupt>,
    rx: Arc<tokio::sync::Mutex<tokio::sync::mpsc::Receiver<Interrupt>>>,
}

impl Default for InterruptQueue {
    fn default() -> Self { Self::new() }
}

impl InterruptQueue {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(64);
        Self { tx, rx: Arc::new(tokio::sync::Mutex::new(rx)) }
    }
    pub fn sender(&self) -> tokio::sync::mpsc::Sender<Interrupt> { self.tx.clone() }
    pub async fn send(&self, i: Interrupt) { let _ = self.tx.send(i).await; }
    async fn drain(&self) -> Vec<Interrupt> {
        let mut rx = self.rx.lock().await;
        let mut out = Vec::new();
        while let Ok(i) = rx.try_recv() { out.push(i); }
        out
    }
}

#[derive(Clone, Debug, Default)]
pub struct KillSwitch {
    killed: Arc<AtomicBool>,
}

impl KillSwitch {
    pub fn kill(&self) { self.killed.store(true, Ordering::SeqCst); }
    pub fn is_killed(&self) -> bool { self.killed.load(Ordering::SeqCst) }
    pub fn reset(&mut self) { self.killed.store(false, Ordering::SeqCst); }
}

/// Map a UI/OS key event to the governed interrupt vocabulary. Actual key
/// capture belongs to the GPUI window / Win32 layer (O0.6/O3.4).
pub fn handle_key(kill: &KillSwitch, q: &InterruptQueue, key: &str) {
    match key {
        "Escape" => { let _ = q.tx.try_send(Interrupt::Soft("user steered via Esc".into())); }
        "Ctrl+Alt+Space" => { kill.kill(); let _ = q.tx.try_send(Interrupt::Kill); }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// The loop
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Phase {
    Perceive,
    Plan,
    Act,
    Reflect,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EffectLog {
    pub step_id: String,
    pub idempotency_key: String,
    pub status: String,
    pub correlation_id: CorrelationId,
    pub ts: String,
    pub no_op: bool,
}

#[derive(Clone, Debug, Default)]
pub struct RunReport {
    pub phases: Vec<Phase>,
    pub effects: Vec<EffectLog>,
    pub injected: Vec<String>,
    pub killed: bool,
}

#[derive(Clone, Debug)]
pub struct LoopConfig {
    pub max_retries: u32,
    pub backoff: Duration,
}

impl Default for LoopConfig {
    fn default() -> Self {
        Self { max_retries: 2, backoff: Duration::from_millis(1) }
    }
}

pub struct AgentLoop {
    pub store: PlanStore,
    pub interrupts: InterruptQueue,
    pub kill: KillSwitch,
    pub config: LoopConfig,
    correlation_id: CorrelationId,
}

impl AgentLoop {
    pub fn new(store: PlanStore) -> Self {
        Self {
            store,
            interrupts: InterruptQueue::new(),
            kill: KillSwitch::default(),
            config: LoopConfig::default(),
            correlation_id: CorrelationId::new(),
        }
    }

    pub fn correlation_id(&self) -> &CorrelationId { &self.correlation_id }

    /// Run a plan. `exec` is the tool boundary; the runtime deliberately knows
    /// nothing about the OS. Authorization is expected to have happened before
    /// `exec` is reached (policy + TOCTOU recheck live in the tool adapter).
    pub async fn run<F>(&mut self, plan: &mut Plan, mut exec: F) -> RunReport
    where
        F: FnMut(&Step) -> ToolResult,
    {
        let mut report = RunReport::default();
        if self.store.save(plan).is_err() {
            return report;
        }

        // Index-based iteration keeps `plan` borrowable for atomic persistence
        // between steps while the executor receives an immutable step view.
        for idx in 0..plan.steps.len() {
            // Kill is checked before and after every await / tool call.
            if self.kill.is_killed() {
                plan.steps[idx].status = StepStatus::Skipped;
                report.killed = true;
                let _ = self.store.save(plan);
                break;
            }

            for interrupt in self.interrupts.drain().await {
                match interrupt {
                    Interrupt::Soft(msg) => report.injected.push(msg),
                    Interrupt::Kill => {
                        self.kill.kill();
                        report.killed = true;
                    }
                }
            }
            if report.killed {
                plan.steps[idx].status = StepStatus::Skipped;
                let _ = self.store.save(plan);
                break;
            }

            // PERCEIVE -> PLAN
            report.phases.push(Phase::Perceive);
            report.phases.push(Phase::Plan);
            plan.steps[idx].status = StepStatus::Running;
            let _ = self.store.save(plan);

            // ACT with bounded retries + idempotency key.
            let step_id = plan.steps[idx].id.clone();
            let command = plan.steps[idx].command.clone();
            let key = idempotency_key(&step_id, &command, &self.correlation_id);
            let mut attempts = 0u32;
            let result = loop {
                attempts += 1;
                report.phases.push(Phase::Act);
                let r = exec(&plan.steps[idx]);
                if r.status == Status::Ok || attempts > self.config.max_retries {
                    break r;
                }
                tokio::time::sleep(self.config.backoff).await;
                if self.kill.is_killed() {
                    report.killed = true;
                    break ToolResult::timeout(&self.correlation_id);
                }
            };

            // REFLECT + receipt/effect log for every outcome, including no-op.
            report.phases.push(Phase::Reflect);
            let status = match result.status {
                Status::Ok => "ok",
                Status::Denied => "denied",
                Status::Timeout => "timeout",
                Status::Error => "error",
            };
            report.effects.push(EffectLog {
                step_id: step_id.clone(),
                idempotency_key: key,
                status: status.into(),
                correlation_id: self.correlation_id.clone(),
                ts: now_rfc3339(),
                no_op: result.stdout.is_empty() && result.artifacts.is_empty(),
            });
            plan.steps[idx].status =
                if result.status == Status::Ok { StepStatus::Done } else { StepStatus::Failed };
            let _ = self.store.save(plan);
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan(n: usize) -> Plan {
        Plan {
            goal: "read-only 5-step task".into(),
            steps: (1..=n)
                .map(|i| Step {
                    id: format!("s{i}"),
                    title: format!("step {i}"),
                    tool: "fs.read".into(),
                    command: format!("Get-Content file{i}.txt"),
                    status: StepStatus::Pending,
                })
                .collect(),
            updated_at: String::new(),
        }
    }

    #[tokio::test]
    async fn five_step_task_completes_and_persists_plan() {
        let dir = tempfile::tempdir().unwrap();
        let mut r#loop = AgentLoop::new(PlanStore::new(dir.path()));
        let mut p = plan(5);
        let report = r#loop.run(&mut p, |_| ToolResult::ok("ok", &CorrelationId::new())).await;
        assert!(!report.killed);
        assert_eq!(report.effects.len(), 5);
        assert!(p.steps.iter().all(|s| s.status == StepStatus::Done));
        assert!(dir.path().join("plan.json").exists());
        let md = std::fs::read_to_string(dir.path().join("todos.md")).unwrap();
        assert!(md.contains("step 5"));
    }

    #[tokio::test]
    async fn kill_stops_inflight_and_logs_completed_effects() {
        let dir = tempfile::tempdir().unwrap();
        let mut r#loop = AgentLoop::new(PlanStore::new(dir.path()));
        let kill = r#loop.kill.clone();
        let mut p = plan(5);
        let report = r#loop
            .run(&mut p, |s| {
                if s.id == "s3" { kill.kill(); }
                ToolResult::ok("ok", &CorrelationId::new())
            })
            .await;
        assert!(report.killed);
        assert_eq!(report.effects.len(), 3);
        assert_eq!(p.steps[3].status, StepStatus::Skipped);
    }

    #[tokio::test]
    async fn soft_interrupt_is_injected_between_tools() {
        let dir = tempfile::tempdir().unwrap();
        let mut r#loop = AgentLoop::new(PlanStore::new(dir.path()));
        handle_key(&r#loop.kill, &r#loop.interrupts, "Escape");
        let mut p = plan(2);
        let report = r#loop.run(&mut p, |_| ToolResult::ok("ok", &CorrelationId::new())).await;
        assert_eq!(report.injected.len(), 1);
    }

    #[tokio::test]
    async fn kill_hotkey_stops_before_first_tool() {
        let dir = tempfile::tempdir().unwrap();
        let mut r#loop = AgentLoop::new(PlanStore::new(dir.path()));
        handle_key(&r#loop.kill, &r#loop.interrupts, "Ctrl+Alt+Space");
        let mut p = plan(3);
        let report = r#loop.run(&mut p, |_| ToolResult::ok("ok", &CorrelationId::new())).await;
        assert!(report.killed);
        assert!(report.effects.is_empty());
    }

    #[test]
    fn idempotency_key_is_stable() {
        let c = CorrelationId::from_string("fixed");
        let a = idempotency_key("s1", "Get-Content x", &c);
        let b = idempotency_key("s1", "Get-Content x", &c);
        assert_eq!(a, b);
        assert_ne!(a, idempotency_key("s2", "Get-Content x", &c));
    }
}
