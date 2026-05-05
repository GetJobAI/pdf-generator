//! Converts plain strings with optional inline markup to Typst content expressions.
//!
//! Recognised markers: `**bold**`, `*bold*`, `_italic_`, `` `code` ``.
//! Unmatched markers are treated as plain text. All other content-mode special
//! characters are escaped, making injection structurally impossible.

#[derive(Debug)]
enum Span<'a> {
    Plain(&'a str),
    Bold(&'a str),
    Italic(&'a str),
    Code(&'a str),
}

/// Converts a string with optional inline markup to a Typst content expression.
///
/// Returns `[text]` for plain strings or a `+` concatenation for mixed content.
pub fn str_to_content(s: &str) -> String {
    if s.is_empty() {
        return "[]".to_owned();
    }

    let spans = parse_spans(s);

    if spans.len() == 1 {
        return render_span(&spans[0]);
    }

    let parts: Vec<String> = spans.iter().map(render_span).collect();
    format!("({})", parts.join(" + "))
}

fn render_span(span: &Span<'_>) -> String {
    match span {
        Span::Plain(s) => format!("[{}]", escape_content(s)),
        Span::Bold(s) => format!("strong[{}]", escape_content(s)),
        Span::Italic(s) => format!("emph[{}]", escape_content(s)),
        Span::Code(s) => format!("raw(\"{}\")", escape_raw(s)),
    }
}

fn parse_spans(s: &str) -> Vec<Span<'_>> {
    let mut spans = Vec::new();
    let mut i = 0;
    let mut plain_start = 0;

    while i < s.len() {
        let rest = &s[i..];

        if rest.starts_with("**") {
            if let Some(off) = rest[2..].find("**") {
                flush_plain(&mut spans, &s[plain_start..i]);
                spans.push(Span::Bold(&rest[2..2 + off]));
                i += 2 + off + 2;
                plain_start = i;
            } else {
                i += 2;
            }
        } else if rest.starts_with('*') {
            if let Some(off) = rest[1..].find('*') {
                flush_plain(&mut spans, &s[plain_start..i]);
                spans.push(Span::Bold(&rest[1..1 + off]));
                i += 1 + off + 1;
                plain_start = i;
            } else {
                i += 1;
            }
        } else if rest.starts_with('_') {
            if let Some(off) = rest[1..].find('_') {
                flush_plain(&mut spans, &s[plain_start..i]);
                spans.push(Span::Italic(&rest[1..1 + off]));
                i += 1 + off + 1;
                plain_start = i;
            } else {
                i += 1;
            }
        } else if rest.starts_with('`') {
            if let Some(off) = rest[1..].find('`') {
                flush_plain(&mut spans, &s[plain_start..i]);
                spans.push(Span::Code(&rest[1..1 + off]));
                i += 1 + off + 1;
                plain_start = i;
            } else {
                i += 1;
            }
        } else {
            // Advance by one full Unicode scalar to avoid splitting multi-byte sequences.
            i += s[i..].chars().next().unwrap().len_utf8();
        }
    }

    flush_plain(&mut spans, &s[plain_start..]);
    spans
}

fn flush_plain<'a>(spans: &mut Vec<Span<'a>>, text: &'a str) {
    if !text.is_empty() {
        spans.push(Span::Plain(text));
    }
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
        assert_eq!(str_to_content("use **Rust**"), "([use ] + strong[Rust])");
    }

    #[test]
    fn single_star_bold() {
        assert_eq!(str_to_content("use *Rust*"), "([use ] + strong[Rust])");
    }

    #[test]
    fn italic() {
        assert_eq!(str_to_content("_note_"), "emph[note]");
    }

    #[test]
    fn code_span() {
        assert_eq!(str_to_content("`cargo test`"), "raw(\"cargo test\")");
    }

    #[test]
    fn mixed_markup() {
        let s = str_to_content("Built *Rust* and _Python_");
        assert!(s.contains("strong[Rust]"));
        assert!(s.contains("emph[Python]"));
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
