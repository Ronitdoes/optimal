use serde::{Deserialize, Serialize};

/// Authorization tiers. Ordered from least to most restrictive so that
/// "most-restrictive decision wins" is simply `Ord::max` (ADR-006).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tier {
    Readonly = 0,
    Auto = 1,
    Confirm = 2,
    Never = 3,
}

impl Tier {
    pub fn most_restrictive(a: Tier, b: Tier) -> Tier {
        a.max(b)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Tier::Readonly => "readonly",
            Tier::Auto => "auto",
            Tier::Confirm => "confirm",
            Tier::Never => "never",
        }
    }

    /// A tier is "governed" if it is not auto-approving. Used to fail closed in
    /// unattended contexts (confirm/never only).
    pub fn is_governed(self) -> bool {
        matches!(self, Tier::Confirm | Tier::Never)
    }
}

/// Deny-by-default: an absent or unknown tier resolves to `never`.
impl Default for Tier {
    fn default() -> Self {
        Tier::Never
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_never() {
        assert_eq!(Tier::default(), Tier::Never);
    }

    #[test]
    fn most_restrictive_wins() {
        assert_eq!(
            Tier::most_restrictive(Tier::Readonly, Tier::Never),
            Tier::Never
        );
        assert_eq!(Tier::most_restrictive(Tier::Auto, Tier::Confirm), Tier::Confirm);
    }

    #[test]
    fn auto_is_not_governed() {
        assert!(!Tier::Auto.is_governed());
        assert!(Tier::Confirm.is_governed());
        assert!(Tier::Never.is_governed());
    }
}
