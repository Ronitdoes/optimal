//! OTel with pinned *internal* span names (ADR-010). Content capture is OFF:
//! the span validator drops any attribute that could carry secret or untrusted
//! content. `correlationId` is the only cross-cutting field.

use std::collections::BTreeMap;

use optimal_common::CorrelationId;
use serde_json::Value;

pub const SPAN_INVOKE_AGENT: &str = "optimal.invoke_agent";
pub const SPAN_CHAT: &str = "optimal.chat";
pub const SPAN_EXECUTE_TOOL: &str = "optimal.execute_tool";

/// Stable internal names mapped to `gen_ai.*` by a pinned adapter (not shipped
/// in O0); dashboards therefore survive semconv renames.
pub const INTERNAL_NAMES: &[&str] = &[SPAN_INVOKE_AGENT, SPAN_CHAT, SPAN_EXECUTE_TOOL];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    pub name: String,
    pub correlation_id: CorrelationId,
    pub attributes: BTreeMap<String, String>,
}

impl Span {
    pub fn new(name: &str, correlation_id: &CorrelationId) -> Self {
        Self { name: name.into(), correlation_id: correlation_id.clone(), attributes: BTreeMap::new() }
    }
}

/// Export-time fail-closed validator. Returns false when a span carries a
/// content-bearing attribute or an unknown internal name.
pub fn validate_span(span: &Span) -> bool {
    if !INTERNAL_NAMES.contains(&span.name.as_str()) {
        return false;
    }
    const FORBIDDEN: &[&str] = &[
        "content", "prompt", "completion", "stdout", "stderr", "payload", "text", "secret", "token", "api_key",
    ];
    !span.attributes.keys().any(|k| FORBIDDEN.iter().any(|f| k.to_ascii_lowercase().contains(f)))
}

pub fn init_subscriber() {
    use tracing_subscriber::EnvFilter;
    // Install once; failure means a subscriber already exists (tests/embedding).
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}

pub fn correlation_from_w3c(traceparent: &str) -> Option<CorrelationId> {
    // W3C traceparent: 00-<32 hex trace>-<16 hex span>-<flags>
    let parts: Vec<&str> = traceparent.split('-').collect();
    if parts.len() == 4 && parts[1].len() == 32 && parts[0] == "00" {
        Some(CorrelationId::from_string(parts[1]))
    } else {
        None
    }
}

pub fn json_attr(value: &Value) -> String {
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_bearing_span_is_dropped() {
        let c = CorrelationId::new();
        let mut s = Span::new(SPAN_EXECUTE_TOOL, &c);
        s.attributes.insert("stdout".into(), "hello".into());
        assert!(!validate_span(&s));
    }

    #[test]
    fn correlation_only_span_passes() {
        let s = Span::new(SPAN_CHAT, &CorrelationId::new());
        assert!(validate_span(&s));
    }

    #[test]
    fn unknown_name_dropped() {
        let s = Span::new("gen_ai.custom_vendor_span", &CorrelationId::new());
        assert!(!validate_span(&s));
    }

    #[test]
    fn parses_traceparent() {
        let tp = "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01";
        assert_eq!(correlation_from_w3c(tp).unwrap().as_str(), "4bf92f3577b34da6a3ce929d0e0e4736");
    }
}
