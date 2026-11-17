//! O0.2 — Tool registry with a *branded* risk declaration.
//!
//! A tool cannot be registered unless its `risk` field is a `RiskDeclaration`.
//! That type has private fields and implements a sealed trait, so no other type
//! can satisfy `BrandedRisk`. Trying to register an unbranded tool is a compile
//! error (enforced by `tests/ui/missing_risk.rs` + trybuild).
//!
//! The uniform result envelope lives in `optimal_common::ToolResult`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use optimal_common::{CorrelationId, Error, Result, ToolResult};
use optimal_policy::Tier;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Branded risk declaration (T3-style private constructor + lint rule).
// ---------------------------------------------------------------------------

mod sealed {
    pub trait Sealed {}
}

/// Sealed marker: only types defined in this crate can implement it.
pub trait BrandedRisk: sealed::Sealed {
    fn reads(&self) -> bool;
    fn writes(&self) -> bool;
    fn acts_as_caller(&self) -> bool;
    fn writes_threads(&self) -> bool;
}

/// Capability bits a tool declares about itself. Private fields force every
/// construction through one of the associated constructors below.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskDeclaration {
    reads: bool,
    writes: bool,
    acts_as_caller: bool,
    writes_threads: bool,
}

impl sealed::Sealed for RiskDeclaration {}
impl BrandedRisk for RiskDeclaration {
    fn reads(&self) -> bool { self.reads }
    fn writes(&self) -> bool { self.writes }
    fn acts_as_caller(&self) -> bool { self.acts_as_caller }
    fn writes_threads(&self) -> bool { self.writes_threads }
}

impl RiskDeclaration {
    pub const fn readonly() -> Self {
        Self { reads: true, writes: false, acts_as_caller: false, writes_threads: false }
    }
    pub const fn writes() -> Self {
        Self { reads: true, writes: true, acts_as_caller: false, writes_threads: false }
    }
    pub const fn acts_as_caller() -> Self {
        Self { reads: true, writes: true, acts_as_caller: true, writes_threads: false }
    }
    pub const fn writes_threads() -> Self {
        Self { reads: true, writes: true, acts_as_caller: false, writes_threads: true }
    }
    pub const fn combine(a: Self, b: Self) -> Self {
        Self {
            reads: a.reads || b.reads,
            writes: a.writes || b.writes,
            acts_as_caller: a.acts_as_caller || b.acts_as_caller,
            writes_threads: a.writes_threads || b.writes_threads,
        }
    }
}

// ---------------------------------------------------------------------------
// Uniform tool envelope.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProbeSpec {
    pub cmd: String,
    pub ttl_ms: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max: u32,
    pub backoff_ms: u64,
}

/// Generic over the risk marker so that an unbranded `R` cannot compile.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolEnvelope<R: BrandedRisk = RiskDeclaration> {
    pub id: String,
    pub domain: String,
    pub risk: R,
    pub permissions: Vec<String>,
    pub approval: String,
    pub default_tier: Tier,
    pub side_effect_level: u8,
    pub idempotent: bool,
    pub dry_run: bool,
    pub timeout_ms: u64,
    pub retries: RetryPolicy,
    pub probe: Option<ProbeSpec>,
    pub correlation_id: CorrelationId,
}

impl ToolEnvelope<RiskDeclaration> {
    /// The only ergonomic constructor; forces a branded declaration.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        domain: impl Into<String>,
        risk: RiskDeclaration,
        default_tier: Tier,
        permissions: &[&str],
    ) -> Self {
        Self {
            id: id.into(),
            domain: domain.into(),
            risk,
            permissions: permissions.iter().map(|s| s.to_string()).collect(),
            approval: "never".into(),
            default_tier,
            side_effect_level: if risk.writes() { 2 } else { 0 },
            idempotent: !risk.writes(),
            dry_run: false,
            timeout_ms: 30_000,
            retries: RetryPolicy { max: 0, backoff_ms: 0 },
            probe: None,
            correlation_id: CorrelationId::new(),
        }
    }

    pub fn with_probe(mut self, cmd: impl Into<String>, ttl_ms: u64) -> Self {
        self.probe = Some(ProbeSpec { cmd: cmd.into(), ttl_ms });
        self
    }

    pub fn with_approval(mut self, approval: impl Into<String>) -> Self {
        self.approval = approval.into();
        self
    }

    /// Unknown, undeclared capabilities resolve to `never` (deny-by-default).
    pub fn unknown(id: impl Into<String>) -> Self {
        Self::new(id, "unknown", RiskDeclaration::readonly(), Tier::Never, &[])
    }
}

/// A tool implementation. Kept as a trait object so the registry is uniform.
pub trait Tool: Send + Sync {
    fn envelope(&self) -> &ToolEnvelope;
    fn invoke(&self, input: &str) -> ToolResult;
}

// ---------------------------------------------------------------------------
// Probe cache: hard TTL + 60s last-good flapping grace + in-flight dedupe.
// (Hermes refinement, phases.md O0.2.)
// ---------------------------------------------------------------------------

const LAST_GOOD_GRACE: Duration = Duration::from_secs(60);

#[derive(Clone, Debug)]
struct ProbeState {
    available: bool,
    last_check: Instant,
    last_good: Option<Instant>,
}

#[derive(Clone, Debug, Default)]
pub struct ProbeCache {
    states: Arc<Mutex<HashMap<String, ProbeState>>>,
    in_flight: Arc<Mutex<Vec<String>>>,
}

impl ProbeCache {
    pub fn new() -> Self { Self::default() }

    /// Is this tool reachable right now? Unknown tools are never reachable.
    pub fn is_available(&self, id: &str) -> bool {
        let states = self.states.lock().unwrap();
        match states.get(id) {
            None => false,
            Some(s) => {
                // The probe runner re-probes every `ProbeSpec::ttl_ms`; the
                // cached verdict plus last-good grace is what gates reachability.
                if s.available {
                    true
                } else if let Some(good) = s.last_good {
                    good.elapsed() <= LAST_GOOD_GRACE // flapping grace
                } else {
                    false
                }
            }
        }
    }

    /// Dedupe concurrent probes of the same tool; returns false if already running.
    pub fn begin_probe(&self, id: &str) -> bool {
        let mut inflight = self.in_flight.lock().unwrap();
        if inflight.iter().any(|x| x == id) { return false; }
        inflight.push(id.to_string());
        true
    }

    pub fn finish_probe(&self, id: &str, ok: bool) {
        self.in_flight.lock().unwrap().retain(|x| x != id);
        let mut states = self.states.lock().unwrap();
        let entry = states.entry(id.to_string()).or_insert(ProbeState {
            available: ok,
            last_check: Instant::now(),
            last_good: if ok { Some(Instant::now()) } else { None },
        });
        entry.available = ok;
        entry.last_check = Instant::now();
        if ok { entry.last_good = Some(Instant::now()); }
    }
}

// ---------------------------------------------------------------------------
// Registry.
// ---------------------------------------------------------------------------

#[derive(Clone, Default)]
pub struct Registry {
    tools: Arc<Mutex<Vec<Arc<dyn Tool>>>>,
    probes: ProbeCache,
}

impl Registry {
    pub fn new() -> Self { Self::default() }

    /// Register a tool. The `ToolEnvelope<RiskDeclaration>` type is the compile-
    /// time proof that a branded declaration exists.
    pub fn register(&self, tool: Arc<dyn Tool>) -> Result<()> {
        let id = tool.envelope().id.clone();
        let mut tools = self.tools.lock().unwrap();
        if tools.iter().any(|t| t.envelope().id == id) {
            return Err(Error::InvalidConfig(format!("duplicate tool id: {id}")));
        }
        tools.push(tool);
        Ok(())
    }

    pub fn probes(&self) -> &ProbeCache { &self.probes }

    /// Invoke a tool by id. A tool with an active probe that reports unavailable
    /// is unreachable: callers never get a handle to execute it.
    pub fn invoke(&self, id: &str, input: &str) -> ToolResult {
        let tool = {
            let tools = self.tools.lock().unwrap();
            tools.iter().find(|t| t.envelope().id == id).cloned()
        };
        let Some(tool) = tool else {
            return ToolResult::denied(
                format!("unknown tool '{id}' (default tier never)"),
                &CorrelationId::new(),
            );
        };
        if let Some(spec) = &tool.envelope().probe {
            if !self.probes.is_available(id) {
                return ToolResult::denied(
                    format!("tool '{id}' unavailable (probe cmd '{}', ttl {}ms)", spec.cmd, spec.ttl_ms),
                    &tool.envelope().correlation_id,
                );
            }
        }
        tool.invoke(input)
    }

    /// `tools.search` meta-tool: discover registered capabilities on demand.
    pub fn search(&self, query: &str) -> Vec<serde_json::Value> {
        let tools = self.tools.lock().unwrap();
        tools
            .iter()
            .filter(|t| t.envelope().id.contains(query) || t.envelope().domain.contains(query))
            .map(|t| {
                let e = t.envelope();
                serde_json::json!({
                    "id": e.id,
                    "domain": e.domain,
                    "default_tier": e.default_tier.as_str(),
                    "permissions": e.permissions,
                    "reads": e.risk.reads(),
                    "writes": e.risk.writes(),
                    "acts_as_caller": e.risk.acts_as_caller(),
                    "writes_threads": e.risk.writes_threads(),
                    "probe": e.probe.as_ref().map(|p| &p.cmd),
                    "available": self.probes.is_available(&e.id) || e.probe.is_none(),
                })
            })
            .collect()
    }
}

/// Helper for tests/simple tools.
pub struct FnTool {
    envelope: ToolEnvelope,
    f: Box<dyn Fn(&str) -> ToolResult + Send + Sync>,
}

impl FnTool {
    pub fn new(envelope: ToolEnvelope, f: impl Fn(&str) -> ToolResult + Send + Sync + 'static) -> Self {
        Self { envelope, f: Box::new(f) }
    }
}

impl Tool for FnTool {
    fn envelope(&self) -> &ToolEnvelope { &self.envelope }
    fn invoke(&self, input: &str) -> ToolResult { (self.f)(input) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env(id: &str) -> ToolEnvelope {
        ToolEnvelope::new(id, "test", RiskDeclaration::readonly(), Tier::Readonly, &["fs:read"])
    }

    #[test]
    fn branded_registration_works() {
        let reg = Registry::new();
        let e = env("fs.read");
        reg.register(Arc::new(FnTool::new(e, |_| ToolResult::ok("ok", &CorrelationId::new())))).unwrap();
        assert_eq!(reg.search("fs").len(), 1);
    }

    #[test]
    fn unknown_tool_is_denied() {
        let reg = Registry::new();
        let r = reg.invoke("does.not.exist", "");
        assert_eq!(r.status, optimal_common::Status::Denied);
    }

    #[test]
    fn unprobed_tool_is_unreachable() {
        let reg = Registry::new();
        let e = env("binary.tool").with_probe("that-binary --version", 30_000);
        reg.register(Arc::new(FnTool::new(e, |_| ToolResult::ok("unreachable", &CorrelationId::new())))).unwrap();
        let r = reg.invoke("binary.tool", "");
        assert_eq!(r.status, optimal_common::Status::Denied);
        assert!(r.stderr.contains("unavailable"));
    }

    #[test]
    fn probe_grace_after_last_good() {
        let p = ProbeCache::new();
        assert!(p.begin_probe("t"));
        assert!(!p.begin_probe("t")); // dedupe
        p.finish_probe("t", true);
        p.finish_probe("t", false); // flapping -> last-good grace
        assert!(p.is_available("t"));
    }

    #[test]
    fn risk_declarations_are_branded_bits() {
        let r = RiskDeclaration::combine(RiskDeclaration::readonly(), RiskDeclaration::writes());
        assert!(r.reads() && r.writes() && !r.acts_as_caller());
    }
}
