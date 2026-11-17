// This MUST fail to compile: `Unbranded` is not the branded `RiskDeclaration`.
use optimal_common::CorrelationId;
use optimal_policy::Tier;
use optimal_registry::{RetryPolicy, ToolEnvelope};

struct Unbranded;

fn main() {
    let _e: ToolEnvelope = ToolEnvelope {
        id: "shell.exec".into(),
        domain: "shell".into(),
        risk: Unbranded, // intentional compile error: not a RiskDeclaration
        permissions: vec![],
        approval: "auto".into(),
        default_tier: Tier::Never,
        side_effect_level: 3,
        idempotent: false,
        dry_run: false,
        timeout_ms: 1_000,
        retries: RetryPolicy { max: 0, backoff_ms: 0 },
        probe: None,
        correlation_id: CorrelationId::new(),
    };
}
