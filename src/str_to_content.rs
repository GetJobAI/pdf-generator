//! Converts plain strings with optional inline markup to Typst content literals.
//!
//! Recognised markers: `**bold**`, `*bold*`, `_italic_`, `` `code` ``.
//! Markers may be nested (e.g. `_**bold italic**_`). Unmatched markers are
//! treated as plain text. All special characters are escaped, making injection
//! structurally impossible.

/// Converts a string with optional inline markup to a Typst content literal `[...]`.
pub fn str_to_content(s: &str) -> String {
    format!("[{}]", render_content(s))
}

/// Renders the body of a content literal (the part between `[` and `]`).
///
/// Called recursively for the inner content of bold/italic spans.
fn render_content(s: &str) -> String {
    let mut out = String::new();
    let mut i = 0;
    let mut plain_start = 0;

    while i < s.len() {
        let rest = &s[i..];

        if let Some(after_open) = rest.strip_prefix("**") {
            if let Some(close) = after_open.find("**") {
                out.push_str(&escape_content(&s[plain_start..i]));
                out.push_str("#strong[");
                out.push_str(&render_content(&after_open[..close]));
                out.push(']');
                i += 2 + close + 2;
                plain_start = i;
            } else {
                i += 2;
            }
        } else if let Some(after_open) = rest.strip_prefix('*') {
            if let Some(close) = after_open.find('*') {
                out.push_str(&escape_content(&s[plain_start..i]));
                out.push_str("#strong[");
                out.push_str(&render_content(&after_open[..close]));
                out.push(']');
                i += 1 + close + 1;
                plain_start = i;
            } else {
                i += 1;
            }
        } else if let Some(after_open) = rest.strip_prefix('_') {
            if let Some(close) = after_open.find('_') {
                out.push_str(&escape_content(&s[plain_start..i]));
                out.push_str("#emph[");
                out.push_str(&render_content(&after_open[..close]));
                out.push(']');
                i += 1 + close + 1;
                plain_start = i;
            } else {
                i += 1;
            }
        } else if let Some(after_open) = rest.strip_prefix('`') {
            if let Some(close) = after_open.find('`') {
                out.push_str(&escape_content(&s[plain_start..i]));
                out.push_str("#raw(\"");
                out.push_str(&escape_raw(&after_open[..close]));
                out.push_str("\")");
                i += 1 + close + 1;
                plain_start = i;
            } else {
                i += 1;
            }
        } else {
            i += s[i..].chars().next().unwrap().len_utf8();
        }
    }

    out.push_str(&escape_content(&s[plain_start..]));
    out
}

/// Escapes characters that are special in Typst content mode (`[...]`).
fn escape_content(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '#' | '[' | ']' | '@' | '_' | '*' | '`' | '\\' | '<' | '$' => {
                out.push('\\');
                out.push(c);
            }
            c => out.push(c),
        }
    }
    out
}

/// Escapes characters special inside a Typst `raw("...")` string literal.
fn escape_raw(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(str_to_content(""), "[]");
    }

    #[test]
    fn plain_string_no_markup() {
        assert_eq!(str_to_content("hello world"), "[hello world]");
    }

    #[test]
    fn double_star_bold() {
        assert_eq!(str_to_content("use **Rust**"), "[use #strong[Rust]]");
    }

    #[test]
    fn single_star_bold() {
        assert_eq!(str_to_content("use *Rust*"), "[use #strong[Rust]]");
    }

    #[test]
    fn italic() {
        assert_eq!(str_to_content("_note_"), "[#emph[note]]");
    }

    #[test]
    fn code_span() {
        assert_eq!(str_to_content("`cargo test`"), "[#raw(\"cargo test\")]");
    }

    #[test]
    fn mixed_markup() {
        let s = str_to_content("Built *Rust* and _Python_");
        assert!(s.contains("#strong[Rust]"));
        assert!(s.contains("#emph[Python]"));
    }

    #[test]
    fn nested_bold_in_italic() {
        assert_eq!(
            str_to_content("_**bold italic code**_"),
            "[#emph[#strong[bold italic]]]"
        );
    }

    #[test]
    fn nested_italic_in_bold() {
        assert_eq!(
            str_to_content("**_bold italic_**"),
            "[#strong[#emph[bold italic]]]"
        );
    }

    #[test]
    fn escapes_hash_in_plain() {
        assert_eq!(str_to_content("C#"), "[C\\#]");
    }

    #[test]
    fn escapes_bracket_in_plain() {
        assert_eq!(str_to_content("a[b]c"), "[a\\[b\\]c]");
    }

    #[test]
    fn escapes_dollar_in_plain() {
        assert_eq!(str_to_content("$100"), "[\\$100]");
    }

    #[test]
    fn unbalanced_star_stays_plain() {
        assert_eq!(str_to_content("price: $5*"), "[price: \\$5\\*]");
    }

    #[test]
    fn unbalanced_backtick_stays_plain() {
        assert_eq!(str_to_content("foo`bar"), "[foo\\`bar]");
    }

    #[test]
    fn unicode_not_split() {
        assert_eq!(str_to_content("héllo"), "[héllo]");
    }
}
