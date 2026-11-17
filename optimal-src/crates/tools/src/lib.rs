//! O0 tool inventory (from `Optimal/tools.md` §1). Each constructor returns a
//! `ToolEnvelope` with a branded declaration and a deny-by-default tier. The
//! OS adapters that implement these tools arrive in O1/O4; the envelope, risk
//! bits, permissions and probe requirements are fixed here from day 1.

use optimal_common::{CorrelationId, ToolResult};
use optimal_policy::Tier;
use optimal_registry::{FnTool, RiskDeclaration, ToolEnvelope};

/// Build the O0 tool set. `env.probe` carries a probe requirement so an
/// unavailable binary makes the tool unreachable (O0.2 gate).
pub fn o0_envelopes() -> Vec<ToolEnvelope> {
    let readonly = RiskDeclaration::readonly();
    let writes = RiskDeclaration::writes();
    let acts = RiskDeclaration::acts_as_caller();

    vec![
        ToolEnvelope::new("fs.read", "fs", readonly, Tier::Readonly, &["fs:read", "net:none", "elevate:false"]),
        ToolEnvelope::new("fs.list", "fs", readonly, Tier::Readonly, &["fs:read", "net:none", "elevate:false"]),
        ToolEnvelope::new("fs.search", "fs", readonly, Tier::Readonly, &["fs:read", "net:none", "elevate:false"]),
        ToolEnvelope::new(
            "env.probe", "env", readonly, Tier::Readonly, &["env:read", "net:none", "elevate:false"],
        )
        .with_probe("optimal-env-probe --version", 30_000),
        ToolEnvelope::new("shell.exec", "shell", acts, Tier::Confirm, &["shell:exec", "net:none", "elevate:false"]),
        ToolEnvelope::new("plan.write", "plan", writes, Tier::Auto, &["fs:write:plan", "net:none", "elevate:false"]),
        ToolEnvelope::new("todo.update", "plan", writes, Tier::Auto, &["fs:write:plan", "net:none", "elevate:false"]),
        ToolEnvelope::new("agent.interrupt", "agent", writes, Tier::Auto, &["agent:control", "elevate:false"]),
        ToolEnvelope::new("agent.kill", "agent", writes, Tier::Auto, &["agent:control", "elevate:false"]),
        ToolEnvelope::new("tools.search", "meta", readonly, Tier::Readonly, &["net:none"]),
        ToolEnvelope::new("policy.check", "policy", readonly, Tier::Readonly, &["net:none"]),
        ToolEnvelope::new("approval.request", "policy", writes, Tier::Auto, &["ui:card", "elevate:false"]),
        ToolEnvelope::new("audit.verify", "audit", readonly, Tier::Readonly, &["fs:read", "net:none"]),
        ToolEnvelope::new("audit.export", "audit", readonly, Tier::Readonly, &["fs:read", "net:none"]),
        ToolEnvelope::new("killswitch.activate", "agent", writes, Tier::Auto, &["agent:control", "elevate:false"]),
    ]
}

/// A stub implementation used by the headless core until OS adapters land.
/// Every call produces a receipt-worthy result; no OS effect occurs.
pub fn stub_tool(id: &'static str) -> FnTool {
    FnTool::new(
        ToolEnvelope::new(id, "stub", RiskDeclaration::readonly(), Tier::Never, &[]),
        move |_| ToolResult::denied(format!("{id} has no OS adapter in O0"), &CorrelationId::new()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use optimal_common::Status;
    use optimal_registry::Tool;

    #[test]
    fn o0_inventory_is_deny_by_default() {
        let tools = o0_envelopes();
        assert_eq!(tools.len(), 15);
        assert!(tools.iter().all(|t| t.default_tier != Tier::Never));
        assert!(tools.iter().any(|t| t.id == "env.probe" && t.probe.is_some()));
    }

    #[test]
    fn unknown_tool_stub_denies() {
        let t = stub_tool("unknown");
        let r = t.invoke("");
        assert_eq!(r.status, Status::Denied);
    }
}
