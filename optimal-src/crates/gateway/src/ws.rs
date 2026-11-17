//! WS upgrade hardening (CVE-2026-53869 DNS-rebinding class).
//! The upgrade is refused unless Host is a loopback authority and Origin is
//! absent or loopback. Fail closed before any PTY / event surface is exposed.

use base64::Engine;
use sha1::{Digest, Sha1};

const WS_GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

pub fn validate_ws_upgrade(host: Option<&str>, origin: Option<&str>) -> Result<(), String> {
    let host = host.ok_or_else(|| "missing Host header".to_string())?;
    let authority = host.rsplit_once(':').map(|(h, _)| h).unwrap_or(host).trim_matches(['[', ']']);
    let host_ok = matches!(authority, "localhost" | "127.0.0.1" | "::1");
    if !host_ok {
        return Err(format!("non-loopback Host rejected: {host}"));
    }

    if let Some(origin) = origin {
        let o = origin.trim().to_ascii_lowercase();
        let allowed = o == "null"
            || o.starts_with("http://localhost")
            || o.starts_with("http://127.0.0.1")
            || o.starts_with("http://[::1]")
            || o.starts_with("https://localhost")
            || o.starts_with("https://127.0.0.1")
            || o.starts_with("https://[::1]");
        if !allowed {
            return Err(format!("foreign Origin rejected: {origin}"));
        }
    }
    Ok(())
}

pub fn accept_key(client_key: &str) -> String {
    let mut h = Sha1::new();
    h.update(client_key.as_bytes());
    h.update(WS_GUID.as_bytes());
    base64::engine::general_purpose::STANDARD.encode(h.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loopback_host_and_absent_origin_allowed() {
        assert!(validate_ws_upgrade(Some("127.0.0.1:8123"), None).is_ok());
        assert!(validate_ws_upgrade(Some("localhost:8123"), None).is_ok());
        assert!(validate_ws_upgrade(Some("[::1]:8123"), None).is_ok());
    }

    #[test]
    fn loopback_origin_allowed() {
        assert!(validate_ws_upgrade(Some("127.0.0.1:8123"), Some("http://localhost:3000")).is_ok());
        assert!(validate_ws_upgrade(Some("127.0.0.1:8123"), Some("null")).is_ok());
    }

    #[test]
    fn dns_rebinding_and_missing_host_rejected() {
        assert!(validate_ws_upgrade(None, None).is_err());
        assert!(validate_ws_upgrade(Some("evil.example:8123"), None).is_err());
        assert!(validate_ws_upgrade(Some("127.0.0.1:8123"), Some("https://evil.example")).is_err());
    }

    #[test]
    fn accept_key_is_rfc6455() {
        // RFC 6455 §1.3 example.
        assert_eq!(accept_key("dGhlIHNhbXBsZSBub25jZQ=="), "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=");
    }
}
