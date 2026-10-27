//! Domain-neutral, error-recovering query syntax parser.
//! Values are strings: there is no validation, type conversion, or execution.
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Query {
    /// Terms in their original order.
    pub terms: Vec<Term>,
}
impl Query {
    pub fn filters(&self) -> impl Iterator<Item = &Filter> {
        self.terms.iter().filter_map(|term| match term { Term::Filter(f) => Some(f), _ => None })
    }
    pub fn text(&self) -> impl Iterator<Item = &Text> {
        self.terms.iter().filter_map(|term| match term { Term::Text(t) => Some(t), _ => None })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term { Text(Text), Filter(Filter) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text { pub value: String, pub span: Range<usize> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Filter {
    pub field: String,
    pub operator: Operator,
    pub value: String,
    pub negated: bool,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator { Match, Eq, Ne, Gt, Ge, Lt, Le }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseResult { pub query: Query, pub diagnostics: Vec<Diagnostic> }
impl ParseResult {
    pub fn is_valid(&self) -> bool { self.diagnostics.is_empty() }
    pub fn into_strict(self) -> Result<Query, Vec<Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.query) } else { Err(self.diagnostics) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity { Warning, Error }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind { IncompleteFilter, UnterminatedQuote, TrailingEscape }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub severity: Severity,
    pub span: Range<usize>,
}

/// Parse all recoverable terms; return diagnostics without discarding other terms.
/// An incomplete term is omitted, never converted to free text.
/// Spans are UTF-8 byte ranges into the original input.
pub fn parse(input: &str) -> ParseResult {
    let mut result = ParseResult { query: Query::default(), diagnostics: Vec::new() };
    let mut chars = input.char_indices().peekable();
    while let Some(&(start, ch)) = chars.peek() {
        if ch.is_whitespace() { chars.next(); continue; }
        let mut decoded = String::new();
        let mut colon = None;
        let mut quote_start = None;
        let mut trailing_escape = None;
        let mut end = start;
        while let Some(&(pos, ch)) = chars.peek() {
            if quote_start.is_none() && ch.is_whitespace() { break; }
            chars.next(); end = pos + ch.len_utf8();
            match ch {
                '"' => { if quote_start.is_some() { quote_start = None; } else { quote_start = Some(pos); } }
                '\\' => {
                    if let Some((at, escaped)) = chars.next() {
                        decoded.push(escaped); end = at + escaped.len_utf8();
                    } else { trailing_escape = Some(pos); break; }
                }
                ':' if quote_start.is_none() => {
                    if colon.is_none() { colon = Some(decoded.len()); }
                    decoded.push(ch);
                }
                _ => decoded.push(ch),
            }
        }
        if let Some(pos) = trailing_escape {
            result.diagnostics.push(Diagnostic { kind: DiagnosticKind::TrailingEscape, severity: Severity::Warning, span: pos..end });
            continue;
        }
        if let Some(pos) = quote_start {
            result.diagnostics.push(Diagnostic { kind: DiagnosticKind::UnterminatedQuote, severity: Severity::Warning, span: pos..end });
            continue;
        }
        let negated = decoded.starts_with('-');
        let body = if negated { &decoded[1..] } else { &decoded };
        if let Some(colon_pos) = colon.and_then(|p| p.checked_sub(usize::from(negated))) {
            if let Some((field, rhs)) = body.get(..colon_pos).zip(body.get(colon_pos + 1..)) {
                if valid_field(field) {
                    let (operator, value) = operator_and_value(rhs);
                    if value.is_empty() {
                        result.diagnostics.push(Diagnostic { kind: DiagnosticKind::IncompleteFilter, severity: Severity::Warning, span: start..end });
                    } else {
                        result.query.terms.push(Term::Filter(Filter {
                            field: field.into(), operator, value: value.into(), negated, span: start..end,
                        }));
                    }
                    continue;
                }
            }
        }
        result.query.terms.push(Term::Text(Text { value: decoded, span: start..end }));
    }
    result
}
fn valid_field(field: &str) -> bool {
    let mut bytes = field.bytes();
    matches!(bytes.next(), Some(b'a'..=b'z' | b'A'..=b'Z'))
        && bytes.all(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-'))
}
fn operator_and_value(rhs: &str) -> (Operator, &str) {
    for (prefix, op) in [("!=", Operator::Ne), (">=", Operator::Ge), ("<=", Operator::Le), ("=", Operator::Eq), (">", Operator::Gt), ("<", Operator::Lt)] {
        if let Some(value) = rhs.strip_prefix(prefix) { return (op, value); }
    }
    (Operator::Match, rhs)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mixed_and_ordered() {
        let result = parse("fix status:open more comments:>10 -is:draft created:>=2026-01-01");
        assert!(result.is_valid());
        assert_eq!(result.query.terms.len(), 6);
        assert_eq!(result.query.text().count(), 2);
        let filters: Vec<_> = result.query.filters().collect();
        assert_eq!(filters.len(), 4);
        assert_eq!(filters[1].operator, Operator::Gt);
        assert_eq!(filters[1].value, "10");
        assert!(filters[2].negated);
        assert_eq!(filters[3].operator, Operator::Ge);
    }
    #[test]
    fn recovery_and_strict() {
        let result = parse("status:open comments:> author:alice");
        assert_eq!(result.query.filters().count(), 2);
        assert_eq!(result.diagnostics[0].kind, DiagnosticKind::IncompleteFilter);
        assert!(result.into_strict().is_err());
    }
    #[test]
    fn quotes_escapes_and_utf8() {
        let s = "žlutý author:\"Jane Doe\" \\\"quoted\\\"";
        let parsed = parse(s);
        assert!(parsed.is_valid());
        assert_eq!(parsed.query.text().next().unwrap().value, "žlutý");
        assert_eq!(parsed.query.filters().next().unwrap().value, "Jane Doe");
        assert_eq!(parsed.query.terms[0].span(), 0..7);
    }
    #[test]
    fn malformed_quote_does_not_destroy_previous_terms() {
        let result = parse("status:open author:\"unfinished");
        assert_eq!(result.query.filters().count(), 1);
        assert_eq!(result.diagnostics[0].kind, DiagnosticKind::UnterminatedQuote);
    }
    #[test]
    fn explicit_operators() {
        for (input, expected) in [("a:=b", Operator::Eq), ("a:!=b", Operator::Ne), ("a:<b", Operator::Lt), ("a:<=b", Operator::Le)] {
            assert_eq!(parse(input).query.filters().next().unwrap().operator, expected);
        }
    }
    impl Term { fn span(&self) -> Range<usize> { match self { Term::Text(t) => t.span.clone(), Term::Filter(f) => f.span.clone() } } }
}
