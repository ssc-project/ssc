use oxc_diagnostics::{LabeledSpan, OxcDiagnostic};
use oxc_span::Span;

#[cold]
pub fn overlong_source() -> OxcDiagnostic {
    OxcDiagnostic::error("Source length exceeds 4 GiB limit")
}

#[cold]
pub fn unexpected_token(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Unexpected token").with_label(span)
}

#[cold]
pub fn expect_token(x0: &str, x1: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error(format!("Expected `{x0}` but found `{x1}`"))
        .with_label(LabeledSpan::new_with_span(Some(format!("`{x0}` expected")), span))
}

#[cold]
pub fn invalid_escape_sequence(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Invalid escape sequence").with_label(span)
}

#[cold]
pub fn unicode_escape_sequence(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Invalid Unicode escape sequence").with_label(span)
}

#[cold]
pub fn invalid_character(x0: char, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error(format!("Invalid Character `{x0}`")).with_label(span)
}

#[cold]
pub fn unterminated_multi_line_comment(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Unterminated multiline comment").with_label(span)
}

#[cold]
pub fn unterminated_string(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Unterminated string").with_label(span)
}

#[cold]
pub fn invalid_css_selector(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Invalid selector").with_label(span)
}

#[cold]
pub fn unexpected_end(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Unexpected end of file").with_label(span)
}
