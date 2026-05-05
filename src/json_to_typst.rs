//! Converts `serde_json::Value` to a valid Typst expression string.
//!
//! String values support inline markup: `**bold**`, `*bold*`, `_italic_`, `` `code` ``.
//! All other content-mode special characters are escaped, making injection structurally
//! impossible as only the explicitly recognised markers produce Typst constructs.

use serde_json::Value;

pub fn serialize(value: &Value) -> String {
    match value {
        Value::Null => "none".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => serialize_string(s),
        Value::Array(arr) => {
            if arr.is_empty() {
                return "()".to_owned();
            }
            let items: Vec<String> = arr.iter().map(serialize).collect();
            // Trailing comma ensures Typst treats single-element lists as arrays.
            format!("({},)", items.join(", "))
        }
        Value::Object(map) => {
            if map.is_empty() {
                return "(:)".to_owned();
            }
            let fields: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("{}: {}", typst_key(k), serialize(v)))
                .collect();
            format!("({})", fields.join(", "))
        }
    }
}

/// Formats a JSON object key as a Typst dict key.
/// Valid identifiers are used bare; anything else is quoted.
fn typst_key(k: &str) -> String {
    if is_typst_ident(k) {
        k.to_owned()
    } else {
        format!("\"{}\"", k.replace('\\', "\\\\").replace('"', "\\\""))
    }
}

fn is_typst_ident(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        }
        _ => false,
    }
}

#[derive(Debug)]
enum Span<'a> {
    Plain(&'a str),
    Bold(&'a str),
    Italic(&'a str),
    Code(&'a str),
}

fn serialize_string(s: &str) -> String {
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
        Span::Plain(t) => format!("[{}]", escape_content(t)),
        Span::Bold(t) => format!("strong[{}]", escape_content(t)),
        Span::Italic(t) => format!("emph[{}]", escape_content(t)),
        Span::Code(t) => format!("raw(\"{}\")", escape_raw(t)),
    }
}

/// Parses a string into a flat list of styled spans.
/// Markers checked in order: `**`, `*`, `_`, `` ` ``.
/// Unmatched markers are treated as plain text.
fn parse_spans(s: &str) -> Vec<Span<'_>> {
    let mut spans = Vec::new();
    let mut i = 0;
    let mut plain_start = 0;

    while i < s.len() {
        let rest = &s[i..];

        if rest.starts_with("**") {
            flush_plain(&mut spans, &s[plain_start..i]);
            if let Some(off) = rest[2..].find("**") {
                spans.push(Span::Bold(&rest[2..2 + off]));
                i += 2 + off + 2;
            } else {
                spans.push(Span::Plain("**"));
                i += 2;
            }
            plain_start = i;
        } else if rest.starts_with('*') {
            flush_plain(&mut spans, &s[plain_start..i]);
            if let Some(off) = rest[1..].find('*') {
                spans.push(Span::Bold(&rest[1..1 + off]));
                i += 1 + off + 1;
            } else {
                spans.push(Span::Plain("*"));
                i += 1;
            }
            plain_start = i;
        } else if rest.starts_with('_') {
            flush_plain(&mut spans, &s[plain_start..i]);
            if let Some(off) = rest[1..].find('_') {
                spans.push(Span::Italic(&rest[1..1 + off]));
                i += 1 + off + 1;
            } else {
                spans.push(Span::Plain("_"));
                i += 1;
            }
            plain_start = i;
        } else if rest.starts_with('`') {
            flush_plain(&mut spans, &s[plain_start..i]);
            if let Some(off) = rest[1..].find('`') {
                spans.push(Span::Code(&rest[1..1 + off]));
                i += 1 + off + 1;
            } else {
                spans.push(Span::Plain("`"));
                i += 1;
            }
            plain_start = i;
        } else {
            // Advance by one full Unicode character to avoid splitting multi-byte sequences.
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

/// Escapes characters that are special in Typst content mode (`[…]`).
fn escape_content(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '#' | '[' | ']' | '@' | '_' | '*' | '`' | '\\' | '<' => {
                out.push('\\');
                out.push(c);
            }
            c => out.push(c),
        }
    }
    out
}

/// Escapes characters special inside a Typst `raw("…")` string literal.
fn escape_raw(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn null_becomes_none() {
        assert_eq!(serialize(&Value::Null), "none");
    }

    #[test]
    fn booleans() {
        assert_eq!(serialize(&json!(true)), "true");
        assert_eq!(serialize(&json!(false)), "false");
    }

    #[test]
    fn empty_array() {
        assert_eq!(serialize(&json!([])), "()");
    }

    #[test]
    fn empty_object() {
        assert_eq!(serialize(&json!({})), "(:)");
    }

    #[test]
    fn single_element_array_has_trailing_comma() {
        assert_eq!(serialize(&json!(["a"])), "([a],)");
    }

    #[test]
    fn plain_string_no_markup() {
        assert_eq!(serialize(&json!("hello world")), "[hello world]");
    }

    #[test]
    fn double_star_bold() {
        assert_eq!(serialize(&json!("use **Rust**")), "([use ] + strong[Rust])");
    }

    #[test]
    fn single_star_bold() {
        assert_eq!(serialize(&json!("use *Rust*")), "([use ] + strong[Rust])");
    }

    #[test]
    fn italic() {
        assert_eq!(serialize(&json!("_note_")), "emph[note]");
    }

    #[test]
    fn code_span() {
        assert_eq!(serialize(&json!("`cargo test`")), "raw(\"cargo test\")");
    }

    #[test]
    fn mixed_markup() {
        let v = json!("Built *Rust* and _Python_");
        let s = serialize(&v);
        assert!(s.contains("strong[Rust]"));
        assert!(s.contains("emph[Python]"));
    }

    #[test]
    fn escapes_hash_in_plain() {
        assert_eq!(serialize(&json!("C#")), "[C\\#]");
    }

    #[test]
    fn escapes_bracket_in_plain() {
        assert_eq!(serialize(&json!("a[b]c")), "[a\\[b\\]c]");
    }

    #[test]
    fn unbalanced_star_is_literal() {
        assert_eq!(serialize(&json!("price: $5*")), "[price\\: \\$5\\*]");
    }

    #[test]
    fn object_serialised_as_typst_dict() {
        let v = json!({"name": "Jane", "age": 30});
        let s = serialize(&v);
        assert!(s.contains("name: [Jane]"));
        assert!(s.contains("age: 30"));
    }

    #[test]
    fn non_ident_key_is_quoted() {
        let v = json!({"-my-key": 1});
        assert!(serialize(&v).contains("\"-my-key\": 1"));
    }
}
