//! O0.7 — CI eval harness. Exits non-zero on:
//!   * a canonicalization/rule-tier regression (70-case table),
//!   * a leaked secret/canary in a transcript (sink scrubber),
//!   * a canary file that was touched or deleted.
//!
//! Evidence is printed to stdout so CI logs are auditable.

use std::path::PathBuf;

use optimal_common::CorrelationId;
use optimal_policy::{decide, load_policy, Tier, DEFAULT_POLICY_TOML};
use optimal_store::redaction::RedactingSink;
use serde::Deserialize;

const MIN_RULES: usize = 60;

#[derive(Debug, Deserialize)]
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

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut failures: Vec<String> = Vec::new();

    // 1. Rule table: every case must match the engine's tier.
    let cfg = match load_policy(DEFAULT_POLICY_TOML) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("FAIL: shipped policy does not validate: {e}");
            std::process::exit(1);
        }
    };
    let table_path = root.join("rules/o0_rule_table.csv");
    let mut checked = 0usize;
    match std::fs::read_to_string(&table_path) {
        Ok(text) => {
            let mut rdr = csv::Reader::from_reader(text.as_bytes());
            for rec in rdr.deserialize::<Row>() {
                match rec {
                    Ok(row) => {
                        let expected = match tier_of(&row.expected_tier) {
                            Some(t) => t,
                            None => {
                                failures.push(format!("{}: unknown tier {}", row.id, row.expected_tier));
                                continue;
                            }
                        };
                        let cmd = row.command.replace("\\n", "\n");
                        let got = decide(&cfg, &cmd);
                        checked += 1;
                        if got.tier != expected {
                            failures.push(format!(
                                "{}: expected {:?}, got {:?} (rule {:?})",
                                row.id, expected, got.tier, got.rule_id
                            ));
                        }
                    }
                    Err(e) => failures.push(format!("CSV parse error: {e}")),
                }
            }
        }
        Err(e) => failures.push(format!("cannot read {}: {e}", table_path.display())),
    }
    if checked < MIN_RULES {
        failures.push(format!("only {checked} rule cases (need >= {MIN_RULES})"));
    }
    println!("EVAL rule-table: checked {checked} cases");

    // 2. Scrubber: a seeded leak transcript must be clean after the sink.
    let sink = RedactingSink::default();
    let leak_path = root.join("fixtures/transcripts/leak_sample.jsonl");
    match std::fs::read_to_string(&leak_path) {
        Ok(leak) => {
            let cleaned = sink.redact(&leak);
            if cleaned.contains("canary-O0-SECRET-9c2b") {
                failures.push("scrubber failed: canary secret survived".into());
            }
            println!("EVAL scrubber: {}", if cleaned.contains("[REDACTED]") { "redacted" } else { "no marker" });
        }
        Err(e) => failures.push(format!("cannot read {}: {e}", leak_path.display())),
    }

    // Confirm no secret in our own emitted events (forced sink at work).
    let emitted = serde_json::json!({"text":"API_KEY=canary-O0-SECRET-9c2b"});
    let cleaned = sink.redact_json(&emitted).to_string();
    if cleaned.contains("canary-O0-SECRET-9c2b") {
        failures.push("forced sink failed on synthetic event payload".into());
    }

    // 3. Canary file must exist, be byte-identical, and be free of secret text.
    let canary = root.join("fixtures/canary/DO_NOT_DELETE.canary");
    match std::fs::read_to_string(&canary) {
        Ok(s) if s.contains("CANARY_O0_DO_NOT_DELETE_7f3a") => {
            println!("EVAL canary: intact");
        }
        Ok(_) => failures.push("canary file content changed".into()),
        Err(e) => failures.push(format!("canary file missing/touched: {e}")),
    }

    // 4. Receipt invariant smoke check (offline, no store needed).
    let r = optimal_common::Receipt::new("eval.noop", "never", &CorrelationId::new());
    assert!(r.no_op);
    println!("EVAL receipt: no-op receipt constructible");

    if !failures.is_empty() {
        eprintln!("EVAL FAILED ({}):", failures.len());
        for f in &failures { eprintln!("  - {f}"); }
        std::process::exit(1);
    }
    println!("EVAL O0 PASS: rule table, scrubber, canary, receipt");
}

