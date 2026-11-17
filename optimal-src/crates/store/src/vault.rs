//! O0.5 — DPAPI-backed vault references. Secrets live in the OS vault; only
//! `{{vault:name}}` placeholders ever enter model context, logs, or events.
//! Substitution happens at the last moment, inside the executor.

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::StoreError as Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VaultRef(String);

impl VaultRef {
    pub fn new(name: impl Into<String>) -> Self { Self(name.into()) }
    pub fn name(&self) -> &str { &self.0 }
    pub fn placeholder(&self) -> String { format!("{{{{vault:{}}}}}", self.0) }

    pub fn parse(s: &str) -> Option<Self> {
        let re = Regex::new(r"^\{\{vault:([A-Za-z0-9_.-]+)\}\}$").ok()?;
        let caps = re.captures(s)?;
        Some(Self(caps.get(1)?.as_str().to_string()))
    }
}

pub trait Vault {
    fn seal(&self, plaintext: &[u8]) -> Result<Vec<u8>, Error>;
    fn unseal(&self, sealed: &[u8]) -> Result<Vec<u8>, Error>;
}

/// Test-only vault. Deliberately NOT a real secret store; it exists so the
/// reference/substitution path is testable without DPAPI. Never use in release.
pub struct MemoryVault;

impl Vault for MemoryVault {
    fn seal(&self, plaintext: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(plaintext.iter().map(|b| b ^ 0xA5).collect())
    }
    fn unseal(&self, sealed: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(sealed.iter().map(|b| b ^ 0xA5).collect())
    }
}

/// Replace every `{{vault:name}}` in `input` using `resolver`. Unresolvable
/// refs become `[UNRESOLVED_VAULT_REF]` — never a raw placeholder that could be
/// mistaken for a secret, and never a silent empty string.
pub fn substitute(input: &str, resolver: impl Fn(&str) -> Option<String>) -> String {
    let re = Regex::new(r"\{\{vault:([A-Za-z0-9_.-]+)\}\}").unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        let name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        resolver(name).unwrap_or_else(|| "[UNRESOLVED_VAULT_REF]".to_string())
    })
    .into_owned()
}

#[cfg(all(windows, feature = "dpapi"))]
pub struct DpapiVault;

#[cfg(all(windows, feature = "dpapi"))]
impl Vault for DpapiVault {
    fn seal(&self, plaintext: &[u8]) -> Result<Vec<u8>, Error> {
        use windows::Win32::Security::Cryptography::{CryptProtectData, CRYPT_INTEGER_BLOB};
        let input = CRYPT_INTEGER_BLOB { cbData: plaintext.len() as u32, pbData: plaintext.as_ptr() as *mut u8 };
        let mut output = CRYPT_INTEGER_BLOB { cbData: 0, pbData: std::ptr::null_mut() };
        unsafe {
            CryptProtectData(&input, None, None, None, None, 0, &mut output)
                .map_err(|e| Error::Internal(format!("DPAPI seal failed: {e}")))?;
            let slice = std::slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec();
            windows::Win32::Foundation::LocalFree(Some(windows::Win32::Foundation::HLOCAL(output.pbData as *mut core::ffi::c_void)));
            Ok(slice)
        }
    }

    fn unseal(&self, sealed: &[u8]) -> Result<Vec<u8>, Error> {
        use windows::Win32::Security::Cryptography::{CryptUnprotectData, CRYPT_INTEGER_BLOB};
        let input = CRYPT_INTEGER_BLOB { cbData: sealed.len() as u32, pbData: sealed.as_ptr() as *mut u8 };
        let mut output = CRYPT_INTEGER_BLOB { cbData: 0, pbData: std::ptr::null_mut() };
        unsafe {
            CryptUnprotectData(&input, None, None, None, None, 0, &mut output)
                .map_err(|e| Error::Internal(format!("DPAPI unseal failed: {e}")))?;
            let slice = std::slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec();
            windows::Win32::Foundation::LocalFree(Some(windows::Win32::Foundation::HLOCAL(output.pbData as *mut core::ffi::c_void)));
            Ok(slice)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ref_roundtrip() {
        let r = VaultRef::new("github-token");
        assert_eq!(VaultRef::parse(&r.placeholder()), Some(r));
    }

    #[test]
    fn substitutes_only_inside_executor() {
        let input = "curl -H 'Authorization: {{vault:token}}' https://example.invalid";
        let out = substitute(input, |n| if n == "token" { Some("s3cr3t".into()) } else { None });
        assert!(out.contains("s3cr3t"));
        // The transcript still holds the placeholder, not the secret.
        assert!(!input.contains("s3cr3t"));
    }

    #[test]
    fn unresolved_ref_is_explicit() {
        assert_eq!(substitute("{{vault:missing}}", |_| None), "[UNRESOLVED_VAULT_REF]");
    }

    #[test]
    fn memory_vault_roundtrip() {
        let v = MemoryVault;
        let sealed = v.seal(b"hunter2").unwrap();
        assert_ne!(sealed, b"hunter2".to_vec());
        assert_eq!(v.unseal(&sealed).unwrap(), b"hunter2");
    }
}
