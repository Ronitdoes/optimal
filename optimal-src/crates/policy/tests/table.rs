//! O0.4 rule-table gate: every case in `evals/rules/o0_rule_table.csv` is run
//! through the real decision engine. Tier assertions are safety-critical;
//! `expected_canonical` is informational (the CSV records a human-readable
//! target convention that the lexical canonicalizer approximates, see
//! `evals/README.md`) and is checked by dedicated canonicalization tests below.

use optimal_policy::{canonicalize, decide, load_policy, Tier, DEFAULT_POLICY_TOML};

const TABLE: &str = include_str!("../../../evals/rules/o0_rule_table.csv");

#[derive(serde::Deserialize)]
struct Row {
    id: String,
    command: String,
    expected_tier: String,
    #[allow(dead_code)]
    expected_canonical: String,
    #[allow(dead_code)]
    notes: String,
}

fn tier_of(s: &str) -> Option<Tier> {
    match s.trim() {
        "readonly" => Some(Tier::Readonly),
        "auto" => Some(Tier::Auto),
        "confirm" => Some(Tier::Confirm),
        "never" => Some(Tier::Never),
        _ => None,
    }
}

#[test]
fn o0_rule_table_has_at_least_60_cases() {
    let mut rdr = csv::Reader::from_reader(TABLE.as_bytes());
    let n = rdr.deserialize::<Row>().count();
    assert!(n >= 60, "rule table has only {n} cases (need >= 60)");
}

#[test]
fn o0_rule_table_all_tiers_match() {
    let cfg = load_policy(DEFAULT_POLICY_TOML).expect("shipped policy must validate");
    let mut rdr = csv::Reader::from_reader(TABLE.as_bytes());
    let mut checked = 0usize;
    let mut failures = Vec::new();

    for rec in rdr.deserialize::<Row>() {
        let row = rec.expect("CSV row must parse");
        let expected = tier_of(&row.expected_tier).unwrap_or_else(|| panic!("bad tier in {}", row.id));
        // One CSV row uses a literal `\n` to denote a newline separator.
        let cmd = row.command.replace("\\n", "\n");
        let got = decide(&cfg, &cmd);
        checked += 1;
        if got.tier != expected {
            failures.push(format!(
                "{}: command={:?} expected={:?} got={:?} (rule={:?}, canonical={:?})",
                row.id, cmd, expected, got.tier, got.rule_id, got.canonical
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "{} tier mismatches:\n{}",
        failures.len(),
        failures.join("\n")
    );
    assert!(checked >= 60, "checked only {checked} rows");
}

#[test]
fn canonicalization_regressions() {
    // Exact outputs of the lexical canonicalizer (not the informational CSV
    // convention). These lock down the alias/slash/8.3/escape behaviour.
    let cases: &[(&str, &str)] = &[
        ("npm i", "npm i"),
        ("PWD", "Get-Location"),
        ("C:/repo/file.txt", "c:\\repo\\file.txt"),
        ("type C:\\repo\\file.txt. ", "Get-Content c:\\repo\\file.txt"),
        ("type C:\\repo\\src\\..\\README.md", "Get-Content c:\\repo\\README.md"),
    ];
    for (input, expected) in cases {
        assert_eq!(canonicalize(input).text, *expected, "canonicalize({input:?})");
    }
}

#[test]
fn canonicalization_flags_aliases_eight_three_unc_and_escape() {
    use optimal_policy::Flag;
    assert!(canonicalize("iwr C:\\PROGRA~1\\x").has(Flag::AliasExpanded));
    assert!(canonicalize("C:\\PROGRA~1\\x").has(Flag::EightDotThree));
    assert!(canonicalize("\\\\srv\\share\\x").has(Flag::Unc));
    assert!(canonicalize("type C:\\..\\..\\x").has(Flag::PathEscape));
    assert!(canonicalize("curl http://169.254.169.254/").has(Flag::MetadataEndpoint));
}
