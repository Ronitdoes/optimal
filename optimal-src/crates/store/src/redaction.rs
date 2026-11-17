//! Forced redaction at the persistence sink (Hermes O0.5 refinement). This is
//! independent of any user toggle: a secret can never opt out of being
//! redacted before it reaches an event row, JSONL mirror, or telemetry span.
//!
//! `{{vault:ref}}` placeholders are *references*, not secrets — they are kept
//! verbatim in transcripts and only substituted inside the executor.

use regex::Regex;
use serde_json::Value;

#[derive(Clone)]
pub struct RedactingSink {
    rules: Vec<(Regex, &'static str)>,
}

impl Default for RedactingSink {
    fn default() -> Self {
        let rules = vec![
            (Regex::new(r#"(?i)\b(api[_-]?key|secret|token|password|passwd|cvv|authorization)\b\s*[:=]\s*[^\s,;"']+"#).unwrap(), "$1=[REDACTED]"),
            (Regex::new(r"canary-[A-Za-z0-9_-]+").unwrap(), "[REDACTED]"),
            (Regex::new(r"CANARY_[A-Z0-9_]+").unwrap(), "[REDACTED]"),
            (Regex::new(r"sk-[A-Za-z0-9]{8,}").unwrap(), "[REDACTED]"),
            (Regex::new(r"gh[pousr]_[A-Za-z0-9]{8,}").unwrap(), "[REDACTED]"),
        ];
        Self { rules }
    }
}

impl RedactingSink {
    /// Redact a free-form string. Returns whether anything changed.
    /// Vault refs are protected first so a rule such as `Authorization={{vault:x}}`
    /// cannot redact the reference away (the executor needs it later).
    pub fn redact_in_place(&self, text: &mut String) -> bool {
        let re_vault = Regex::new(r"\{\{vault:[A-Za-z0-9_.-]+\}\}").unwrap();
        let refs: Vec<String> = re_vault.find_iter(text).map(|m| m.as_str().to_string()).collect();
        let mut saved = Vec::new();
        for (i, r) in refs.iter().enumerate() {
            // Leading ',' is excluded by the secret-value character class, so
            // a rule cannot swallow the placeholder while redacting a key.
            let token = format!(",VAULTREF{i},");
            *text = text.replace(r, &token);
            saved.push((token, r.clone()));
        }

        let mut changed = false;
        for (re, replacement) in &self.rules {
            if re.is_match(text) {
                *text = re.replace_all(text, *replacement).into_owned();
                changed = true;
            }
        }
        for (token, original) in saved {
            *text = text.replace(&token, &original);
        }
        changed
    }

    pub fn redact(&self, text: &str) -> String {
        let mut s = text.to_string();
        self.redact_in_place(&mut s);
        s
    }

    /// Recursively redact every string in a JSON value (event payloads).
    pub fn redact_value(&self, value: &mut Value) -> bool {
        match value {
            Value::String(s) => self.redact_in_place(s),
            // Explicit loops (not `any`) so every value is visited even after
            // the first redaction; short-circuiting would leak later secrets.
            Value::Array(a) => {
                let mut changed = false;
                for v in a { changed |= self.redact_value(v); }
                changed
            }
            Value::Object(o) => {
                let mut changed = false;
                for v in o.values_mut() { changed |= self.redact_value(v); }
                changed
            }
            _ => false,
        }
    }

    pub fn redact_json(&self, value: &Value) -> Value {
        let mut v = value.clone();
        self.redact_value(&mut v);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn redacts_api_key_and_canary() {
        let sink = RedactingSink::default();
        let s = sink.redact("API_KEY=canary-O0-SECRET-9c2b and CANARY_O0_DO_NOT_DELETE_7f3a");
        assert!(!s.contains("canary-O0-SECRET"));
        assert!(!s.contains("CANARY_O0_DO_NOT_DELETE"));
        assert!(s.contains("[REDACTED]"));
    }

    #[test]
    fn redacts_nested_json() {
        let sink = RedactingSink::default();
        let v = sink.redact_json(&json!({"a": {"msg": "password=hunter2"}, "b": ["token: abc123XY"]}));
        let s = v.to_string();
        assert!(!s.contains("hunter2"));
        assert!(!s.contains("abc123XY"));
    }

    #[test]
    fn keeps_vault_refs_verbatim() {
        let sink = RedactingSink::default();
        let s = sink.redact("Authorization={{vault:github-token}}");
        // The key name is redacted but the ref survives so the executor can
        // resolve it at the last moment.
        assert!(s.contains("{{vault:github-token}}"));
    }
}
