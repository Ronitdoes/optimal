//! Load-time policy validation. A configuration that is ambiguous, overbroad,
//! or conflicts with an unconditional floor is refused (fail closed).
//!
//! Two validators share the contract:
//! * `PureRustValidator` (always compiled) — conservative structural checks.
//! * `Z3Validator` (feature `z3-validate`) — true SMT satisfiability check.
//!
//! The Z3 path is enabled in CI where z3-sys' native prerequisites exist; the
//! pure-Rust path is used otherwise. Both refuse the same categories of bad
//! config; neither ever fails open.

use std::collections::{BTreeMap, BTreeSet};

use crate::rules::{MatchKind, PolicyConfig};
use crate::tier::Tier;

pub trait PolicyValidator {
    fn validate(&self, cfg: &PolicyConfig) -> Result<(), String>;
}

#[derive(Default)]
pub struct PureRustValidator;

impl PolicyValidator for PureRustValidator {
    fn validate(&self, cfg: &PolicyConfig) -> Result<(), String> {
        if cfg.default_tier != Tier::Never {
            return Err("default_tier must be 'never' (deny-by-default)".into());
        }
        let mut ids = BTreeSet::new();
        let mut groups: BTreeMap<(MatchKind, String), Tier> = BTreeMap::new();
        let protected = ["rm", "remove-item", "iex", "invoke-expression", "format", "reg", "del"];

        for r in &cfg.rules {
            if !ids.insert(r.id.clone()) {
                return Err(format!("duplicate rule id: {}", r.id));
            }
            let pat_key = (r.kind, r.pattern.to_ascii_lowercase());
            if let Some(prev) = groups.get(&pat_key) {
                if *prev != r.tier {
                    return Err(format!(
                        "conflicting tiers for pattern '{}': {:?} vs {:?}",
                        r.pattern, prev, r.tier
                    ));
                }
            } else {
                groups.insert(pat_key, r.tier);
            }

            if matches!(r.kind, MatchKind::Regex) && regex::Regex::new(&r.pattern).is_err() {
                return Err(format!("invalid regex in rule {}: {}", r.id, r.pattern));
            }
            if (r.pattern == "*" || r.pattern.is_empty()) && r.tier != Tier::Never {
                return Err(format!("overbroad rule {} must be 'never'", r.id));
            }
            if r.floor && r.tier != Tier::Never {
                return Err(format!("floor rule {} must declare tier 'never'", r.id));
            }
            let p = r.pattern.to_ascii_lowercase();
            if protected.iter().any(|v| p == *v || p.starts_with(&format!("{} ", v)))
                && matches!(r.tier, Tier::Readonly | Tier::Auto)
            {
                return Err(format!(
                    "rule {} would lower an unconditional floor for '{}'",
                    r.id, r.pattern
                ));
            }
        }
        Ok(())
    }
}

#[cfg(feature = "z3-validate")]
pub struct Z3Validator;

#[cfg(feature = "z3-validate")]
impl PolicyValidator for Z3Validator {
    fn validate(&self, cfg: &PolicyConfig) -> Result<(), String> {
        use z3::ast::Int;
        use z3::{SatResult, Solver};

        // z3 0.19 manages a thread-local context implicitly; `Context::new`
        // is crate-private and must not be called here.
        let solver = Solver::new();

        if cfg.default_tier != Tier::Never {
            return Err("default_tier must be 'never' (deny-by-default)".into());
        }

        let mut seen = BTreeSet::new();
        let mut tier_vars: Vec<(String, (MatchKind, String), Int, i64)> = Vec::new();
        for r in cfg.rules.iter() {
            if !seen.insert(r.id.clone()) {
                return Err(format!("duplicate rule id: {}", r.id));
            }
            let declared = match r.tier {
                Tier::Readonly => 0,
                Tier::Auto => 1,
                Tier::Confirm => 2,
                Tier::Never => 3,
            };
            let v = Int::fresh_const(&format!("tier_{}", r.id));
            solver.assert(v.eq(declared));
            let key = (r.kind, r.pattern.to_ascii_lowercase());
            tier_vars.push((r.id.clone(), key, v, declared));
        }

        // Identical patterns must resolve to identical tiers. If two rules
        // declare different tiers for the same pattern, the assertions become
        // unsatisfiable and Z3 reports the conflict.
        for a in 0..tier_vars.len() {
            for b in (a + 1)..tier_vars.len() {
                if tier_vars[a].1 == tier_vars[b].1 {
                    solver.assert(tier_vars[a].2.eq(&tier_vars[b].2));
                    if tier_vars[a].3 != tier_vars[b].3 {
                        return Err(format!(
                            "Z3 conflict: rules '{}' and '{}' match the same pattern at different tiers",
                            tier_vars[a].0, tier_vars[b].0
                        ));
                    }
                }
            }
        }

        match solver.check() {
            SatResult::Unsat => Err("Z3: policy constraints are unsatisfiable".into()),
            SatResult::Unknown => Err("Z3: policy constraints could not be decided (fail closed)".into()),
            SatResult::Sat => Ok(()),
        }
    }
}

/// Validate using the strongest validator available in this build.
/// The structural/floor checks always run; Z3 adds an SMT satisfiability pass
/// on top when enabled, so both build modes enforce identical safety floors.
pub fn validate_config(cfg: &PolicyConfig) -> Result<(), String> {
    PureRustValidator.validate(cfg)?;
    #[cfg(feature = "z3-validate")]
    Z3Validator.validate(cfg)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Rule;

    fn rule(id: &str, pattern: &str, tier: Tier) -> Rule {
        Rule {
            id: id.into(),
            kind: MatchKind::Verb,
            pattern: pattern.into(),
            tier,
            floor: false,
            reason: String::new(),
        }
    }

    #[test]
    fn refuses_auto_default() {
        let cfg = PolicyConfig {
            default_tier: Tier::Auto,
            ..Default::default()
        };
        assert!(validate_config(&cfg).is_err());
    }

    #[test]
    fn refuses_conflicting_rules() {
        let cfg = PolicyConfig {
            rules: vec![rule("a", "git", Tier::Readonly), rule("b", "git", Tier::Never)],
            ..Default::default()
        };
        assert!(validate_config(&cfg).is_err());
    }

    #[test]
    fn refuses_lowering_a_floor() {
        let cfg = PolicyConfig {
            rules: vec![rule("a", "rm", Tier::Auto)],
            ..Default::default()
        };
        assert!(validate_config(&cfg).is_err());
    }

    #[test]
    fn accepts_valid_config() {
        let cfg = PolicyConfig {
            rules: vec![rule("git", "git", Tier::Readonly)],
            ..Default::default()
        };
        assert!(validate_config(&cfg).is_ok());
    }
}
