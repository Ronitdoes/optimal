//! Guardian client (O0 stub) — ADVISORY ONLY. A reviewer verdict can never
//! grant an approval; deterministic policy remains the boundary (ADR-014/019).

use optimal_common::{CorrelationId, ToolResult};
use optimal_policy::{decide, PolicyConfig, Tier};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Advisory {
    Approve,
    Deny,
    Unavailable,
}

pub trait Reviewer {
    fn review(&self, command: &str, correlation_id: &CorrelationId) -> Advisory;
}

/// Deterministic advisory reviewer: mirrors the policy decision but has no
/// authority to change it. A model-backed reviewer plugs in behind `Reviewer`
/// and is still advisory.
pub struct PolicyReviewer<'a> {
    pub policy: &'a PolicyConfig,
}

impl Reviewer for PolicyReviewer<'_> {
    fn review(&self, command: &str, _correlation_id: &CorrelationId) -> Advisory {
        match decide(self.policy, command).tier {
            Tier::Never => Advisory::Deny,
            _ => Advisory::Approve,
        }
    }
}

/// Apply the advisory verdict without ever lowering the policy tier. This is
/// the only function allowed to combine a review with a decision.
pub fn combine(policy_tier: Tier, advisory: Advisory, correlation_id: &CorrelationId) -> ToolResult {
    match advisory {
        Advisory::Deny | Advisory::Unavailable => {
            ToolResult::denied("advisory reviewer withheld (deny/unavailable); policy still applies", correlation_id)
        }
        Advisory::Approve => {
            if policy_tier == Tier::Never {
                ToolResult::denied("policy tier is never; guardian approval cannot override", correlation_id)
            } else {
                ToolResult::ok("advisory reviewed", correlation_id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use optimal_common::Status;

    #[test]
    fn guardian_cannot_grant_when_policy_is_never() {
        let r = combine(Tier::Never, Advisory::Approve, &CorrelationId::new());
        assert_eq!(r.status, Status::Denied);
    }

    #[test]
    fn unavailable_reviewer_fails_closed() {
        let r = combine(Tier::Confirm, Advisory::Unavailable, &CorrelationId::new());
        assert_eq!(r.status, Status::Denied);
    }
}
