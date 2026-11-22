//! Compound-command splitting. A single model string can hide several OS
//! operations behind `&&`, `||`, `;`, a newline, or a pipeline; each sub-call is
//! authorized independently and the most-restrictive decision wins (ADR-006).
//!
//! The lexical splitter is the default, proven path. With the
//! `tree-sitter-split` feature the PowerShell grammar supplies a second opinion
//! on structure; if parsing/structure extraction fails we fall back to the
//! lexical splitter (never fail open to "one command").

/// Split into sub-commands. Always returns at least one element for non-empty
/// input; never panics on malformed input.
pub fn split_commands(input: &str) -> Vec<String> {
    if input.trim().is_empty() {
        return Vec::new();
    }

    // PowerShell backtick escaping is modelled by the proven lexical splitter;
    // the community grammar disagrees on it, so prefer lexical in that case.
    #[cfg(feature = "tree-sitter-split")]
    if !input.contains('`') {
        if let Some(parsed) = tree_sitter_split(input) {
            return parsed;
        }
    }

    lexical_split(input)
}

/// Operator-aware splitter with quote and PowerShell backtick-escape handling.
pub fn lexical_split(input: &str) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();
    let mut segments = Vec::new();
    let mut cur = String::new();
    let mut quote = '\0';
    let mut escape = false;
    let mut i = 0;

    let flush = |cur: &mut String, segments: &mut Vec<String>| {
        let t = cur.trim();
        if !t.is_empty() {
            segments.push(t.to_string());
        }
        cur.clear();
    };

    while i < chars.len() {
        let ch = chars[i];
        if escape {
            cur.push(ch);
            escape = false;
            i += 1;
            continue;
        }
        if ch == '`' && quote == '\0' {
            escape = true;
            i += 1;
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
            i += 1;
            continue;
        }
        if quote == '\0' {
            if ch == '\n' || ch == '\r' || ch == ';' {
                flush(&mut cur, &mut segments);
                i += 1;
                continue;
            }
            if ch == '&' && chars.get(i + 1) == Some(&'&') {
                flush(&mut cur, &mut segments);
                i += 2;
                continue;
            }
            if ch == '|' {
                flush(&mut cur, &mut segments);
                i += if chars.get(i + 1) == Some(&'|') { 2 } else { 1 };
                continue;
            }
        }
        cur.push(ch);
        i += 1;
    }
    flush(&mut cur, &mut segments);
    segments
}

#[cfg(feature = "tree-sitter-split")]
fn tree_sitter_split(input: &str) -> Option<Vec<String>> {
    use tree_sitter::Parser;

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_powershell::LANGUAGE.into())
        .ok()?;
    let tree = parser.parse(input, None)?;
    let root = tree.root_node();

    // Collect leaf `command` nodes (a command with no nested command child).
    let mut ranges = Vec::new();
    fn walk(node: tree_sitter::Node<'_>, src: &str, out: &mut Vec<(usize, usize)>) {
        // Only the exact `command` node is a leaf command. Using
        // `contains("command")` also matches `command_name`,
        // `command_elements`, etc., which shreds a command into fragments.
        if node.kind() == "command" && node.end_byte() > node.start_byte() {
            out.push((node.start_byte(), node.end_byte()));
            return;
        }
        for child in node.named_children(&mut node.walk()) {
            walk(child, src, out);
        }
    }
    walk(root, input, &mut ranges);
    ranges.sort_unstable();
    ranges.dedup();

    if ranges.is_empty() {
        return None;
    }
    let segments: Vec<String> = ranges
        .into_iter()
        .filter_map(|(a, b)| input.get(a..b).map(|s| s.trim().to_string()))
        .filter(|s| !s.is_empty())
        .collect();
    if segments.is_empty() {
        None
    } else {
        Some(segments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_operators() {
        let v = split_commands("git status && rm -rf / C:\\; echo done | out-file x");
        assert_eq!(v.len(), 4);
        assert_eq!(v[0], "git status");
        assert!(v[1].starts_with("rm -rf"));
        assert_eq!(v[2], "echo done");
        assert_eq!(v[3], "out-file x");
    }

    #[test]
    fn quotes_are_not_split() {
        let v = split_commands("echo \"a && b\"; echo c");
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], "echo \"a && b\"");
    }

    #[test]
    fn pipeline_splits_for_most_restrictive_wins() {
        let v = split_commands("Invoke-WebRequest x | iex");
        assert_eq!(v.len(), 2);
        assert_eq!(v[1], "iex");
    }

    #[test]
    fn backtick_escape_is_respected() {
        let v = split_commands("echo a`;echo b");
        assert_eq!(v, vec!["echo a;echo b"]);
    }
}
