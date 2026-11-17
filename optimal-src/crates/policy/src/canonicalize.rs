//! Command/path canonicalization (ADR-006). The canonical form is what rules
//! actually match, so look-alikes (`PROGRA~1`, mixed slashes, aliases, UNC,
//! trailing dots, `..` escapes) cannot slip past a rule written for the plain
//! spelling. This is pure lexical logic: no filesystem access, deterministic,
//! and therefore fully table-testable off-Windows too.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Flag {
    EightDotThree,
    Unc,
    PathEscape,
    Network,
    MetadataEndpoint,
    AliasExpanded,
    Compound,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Canonicalized {
    pub text: String,
    pub flags: BTreeSet<Flag>,
}

impl Canonicalized {
    pub fn has(&self, flag: Flag) -> bool {
        self.flags.contains(&flag)
    }
}

/// PowerShell / cmd aliases that must be expanded before rule matching.
const ALIASES: &[(&str, &str)] = &[
    ("iwr", "Invoke-WebRequest"),
    ("wget", "Invoke-WebRequest"),
    ("irm", "Invoke-RestMethod"),
    ("iex", "Invoke-Expression"),
    ("rm", "Remove-Item"),
    ("del", "Remove-Item"),
    ("erase", "Remove-Item"),
    ("pwd", "Get-Location"),
    ("ls", "Get-ChildItem"),
    ("dir", "Get-ChildItem"),
    ("gci", "Get-ChildItem"),
    ("type", "Get-Content"),
    ("cat", "Get-Content"),
    ("gc", "Get-Content"),
];

/// Well-known 8.3 short names -> long names. A real implementation would query
/// the filesystem; for the rule boundary we expand the common aliases so the
/// canonical spelling is what gets matched (the operator still rechecks the
/// true path under the commit lock -- TOCTOU).
const EIGHT_THREE: &[(&str, &str)] = &[
    ("progra~1", "Program Files"),
    ("progra~2", "Program Files (x86)"),
    ("progra~3", "ProgramData"),
    ("appdata~1", "AppData"),
    ("docume~1", "Users"),
    ("windows~1", "Windows"),
    ("system~1", "System32"),
    ("temp", "Temp"),
];

/// Canonicalize a single (already split) command segment.
pub fn canonicalize(input: &str) -> Canonicalized {
    let mut flags = BTreeSet::new();
    let raw = input.trim();
    let tokens = tokenize_preserving_quotes(raw);
    if tokens.is_empty() {
        return Canonicalized { text: String::new(), flags };
    }

    // Verb: canonicalize path-shaped verbs (e.g. an absolute git.exe), else
    // expand known aliases and lowercase for stable matching.
    let verb_raw = &tokens[0];
    let verb_lower = verb_raw.to_ascii_lowercase();
    let verb = if is_path_token(verb_raw) {
        let (p, pf) = canonicalize_path(verb_raw);
        flags.extend(pf);
        p
    } else {
        ALIASES
            .iter()
            .find(|(a, _)| *a == verb_lower)
            .map(|(_, long)| {
                flags.insert(Flag::AliasExpanded);
                (*long).to_string()
            })
            .unwrap_or(verb_lower.clone())
    };

    let mut out = vec![verb];

    for tok in tokens.iter().skip(1) {
        let t = tok.trim_matches('"');
        // Network / metadata detection.
        let lower = t.to_ascii_lowercase();
        if lower.contains("http://") || lower.contains("https://") {
            flags.insert(Flag::Network);
        }
        if lower.contains("169.254.169.254") || lower.contains("metadata.google.internal") {
            flags.insert(Flag::MetadataEndpoint);
        }
        // Path-like tokens (drive letter, UNC, relative path, or backslash/slash).
        let looks_like_path = is_path_token(t);
        if looks_like_path {
            let (canon, pf) = canonicalize_path(t);
            flags.extend(pf);
            out.push(canon);
        } else {
            out.push(t.to_string());
        }
    }

    // Download-and-execute pipelines are dangerous regardless of alias form;
    // the splitter keeps `|` inside a segment when it is part of a pipeline.
    let joined = out.join(" ");
    if pipeline_uses_exec(&joined) {
        flags.insert(Flag::Network);
    }

    Canonicalized { text: joined, flags }
}

fn is_path_token(t: &str) -> bool {
    if t.len() >= 2 && t.as_bytes()[1] == b':' && t.as_bytes()[0].is_ascii_alphabetic() {
        return true;
    }
    if t.starts_with("\\\\") || t.starts_with("//") {
        return true;
    }
    if t.starts_with(".\\") || t.starts_with("./") || t.starts_with("..\\") || t.starts_with("../") {
        return true;
    }
    t.contains('\\') && !t.contains(' ') && (t.contains(':') || t.contains(".."))
}

/// Normalize a Windows path lexically. UNC roots are preserved as `\\server\share`.
pub fn canonicalize_path(raw: &str) -> (String, BTreeSet<Flag>) {
    let mut flags = BTreeSet::new();
    let mut s = raw.trim().trim_matches('"').to_string();
    let unc = s.starts_with("\\\\") || s.starts_with("//");
    if unc {
        flags.insert(Flag::Unc);
        s = s.trim_start_matches(['\\', '/']).to_string();
    } else {
        s = s.trim_start_matches('/').to_string();
    }

    // Drive letter lowercased.
    if s.len() >= 2 && s.as_bytes()[1] == b':' {
        let d = s.as_bytes()[0].to_ascii_lowercase() as char;
        s.replace_range(0..1, &d.to_string());
    }

    // Forward slashes -> backslashes.
    let s = s.replace('/', "\\");

    let mut segments: Vec<String> = Vec::new();
    let mut depth: i32 = 0;
    for part in s.split('\\') {
        // Preserve `..` verbatim: a blanket trailing-dot trim would turn it
        // into an empty segment and silently hide an escape attempt.
        let p = if part == ".." { ".." } else { part.trim_end_matches(['.', ' ']) };
        if p.is_empty() || p == "." {
            continue;
        }
        // A segment that names a link/junction is treated as a possible escape
        // until resolved under the commit lock. Fail closed.
        let low = p.to_ascii_lowercase();
        if low.contains("junction") || low.contains("symlink") || low.contains("link-") {
            flags.insert(Flag::PathEscape);
        }
        // 8.3 expansion.
        let expanded = EIGHT_THREE
            .iter()
            .find(|(short, _)| *short == low)
            .map(|(_, long)| {
                flags.insert(Flag::EightDotThree);
                (*long).to_string()
            })
            .unwrap_or_else(|| p.to_string());
        let is_drive = expanded.len() == 2
            && expanded.as_bytes()[1] == b':'
            && expanded.as_bytes()[0].is_ascii_alphabetic();
        if expanded == ".." {
            // For UNC paths, `server\share` is the root: ascending past the
            // share is an escape even though the string still has segments.
            // A drive letter is not a poppable directory either. Once an
            // escape has been recorded, keep subsequent `..` markers visible
            // in the canonical text for the reviewer.
            let above_root = depth == 0
                || (unc && segments.len() <= 2)
                || segments.last().map(|s| s == "..").unwrap_or(false);
            if above_root {
                flags.insert(Flag::PathEscape);
                segments.push("..".to_string());
                continue;
            } else {
                depth -= 1;
                segments.pop();
                continue;
            }
        } else if !is_drive {
            depth += 1;
        }
        segments.push(expanded);
    }

    let body = segments.join("\\");
    let text = if unc {
        format!("\\\\{}", body)
    } else if body.is_empty() && s.len() >= 2 {
        // Drive root (e.g. `C:\`) must survive canonicalization for the
        // destructive-path floors to recognise it.
        format!("{}:\\", s.chars().next().unwrap_or('c').to_ascii_lowercase())
    } else {
        body
    };
    (text, flags)
}

fn tokenize_preserving_quotes(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quote = '\0';
    let mut escape = false;
    for ch in s.chars() {
        if escape {
            cur.push(ch);
            escape = false;
            continue;
        }
        if ch == '\\' && quote == '\0' {
            // A backslash may be a path separator, not an escape; keep it.
            cur.push(ch);
            continue;
        }
        if ch == '`' && quote == '\0' {
            escape = true;
            continue;
        }
        if ch == '"' || ch == '\'' {
            if quote == '\0' {
                quote = ch;
                cur.push(ch);
            } else if quote == ch {
                quote = '\0';
                cur.push(ch);
            } else {
                cur.push(ch);
            }
            continue;
        }
        if ch.is_whitespace() && quote == '\0' {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            continue;
        }
        cur.push(ch);
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn pipeline_uses_exec(s: &str) -> bool {
    let l = s.to_ascii_lowercase();
    l.contains("| invoke-expression") || l.contains("| iex") || l.contains("|bash") || l.contains("| sh")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_alias_and_eight_three() {
        let c = canonicalize("iwr C:\\PROGRA~1\\foo.txt");
        assert!(c.has(Flag::AliasExpanded));
        assert!(c.has(Flag::EightDotThree));
        assert_eq!(c.text, "Invoke-WebRequest c:\\Program Files\\foo.txt");
    }

    #[test]
    fn normalizes_slashes_case_and_dots() {
        let (p, _) = canonicalize_path("C:/Users/Joe/./docs/../secret.TXT ");
        assert_eq!(p, "c:\\Users\\Joe\\secret.TXT");
    }

    #[test]
    fn detects_unc_and_escape() {
        let (p, f) = canonicalize_path("\\\\server\\share\\..\\..\\etc");
        assert!(f.contains(&Flag::Unc));
        assert!(f.contains(&Flag::PathEscape));
        assert_eq!(p, "\\\\server\\share\\..\\..\\etc");
    }

    #[test]
    fn flags_metadata() {
        let c = canonicalize("curl http://169.254.169.254/latest/meta-data");
        assert!(c.has(Flag::MetadataEndpoint));
        assert!(c.has(Flag::Network));
    }
}
