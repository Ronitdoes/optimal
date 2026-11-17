use serde::{Deserialize, Serialize};

use crate::tier::Tier;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchKind {
    #[default]
    Verb,
    Regex,
    PathPrefix,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    #[serde(default)]
    pub kind: MatchKind,
    pub pattern: String,
    pub tier: Tier,
    /// Unconditional floor: cannot be lowered by any other rule or bypass.
    #[serde(default)]
    pub floor: bool,
    #[serde(default)]
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyConfig {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub rules: Vec<Rule>,
    /// Missing/unknown default resolves to `never` via `Tier::default`.
    #[serde(default)]
    pub default_tier: Tier,
}

fn default_version() -> u32 {
    1
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            version: 1,
            rules: Vec::new(),
            default_tier: Tier::Never,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_default_tier_is_never() {
        let cfg: PolicyConfig = toml::from_str("version = 1\n").unwrap();
        assert_eq!(cfg.default_tier, Tier::Never);
    }

    #[test]
    fn parses_rules() {
        let cfg: PolicyConfig = toml::from_str(
            r#"
            version = 1
            default_tier = "never"
            [[rules]]
            id = "git-readonly"
            kind = "verb"
            pattern = "git"
            tier = "readonly"
            "#,
        )
        .unwrap();
        assert_eq!(cfg.rules.len(), 1);
        assert_eq!(cfg.rules[0].tier, Tier::Readonly);
        assert_eq!(cfg.rules[0].kind, MatchKind::Verb);
    }
}
