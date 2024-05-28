use oxc_diagnostics::{LabeledSpan, OxcDiagnostic};
use oxc_span::Span;

#[cold]
pub fn overlong_source() -> OxcDiagnostic {
    OxcDiagnostic::error("Source length exceeds 4 GiB limit")
}

#[cold]
pub fn unexpected_token(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Unexpected token").with_labels([span0.into()])
}

#[cold]
pub fn expect_token(x0: &str, x1: &str, span2: Span) -> OxcDiagnostic {
    OxcDiagnostic::error(format!("Expected `{x0}` but found `{x1}`"))
        .with_labels([LabeledSpan::new_with_span(Some(format!("`{x0}` expected")), span2)])
}

#[cold]
pub fn invalid_escape_sequence(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Invalid escape sequence").with_labels([span0.into()])
}

#[cold]
pub fn unicode_escape_sequence(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Invalid Unicode escape sequence").with_labels([span0.into()])
}

#[cold]
pub fn invalid_character(x0: char, span1: Span) -> OxcDiagnostic {
    OxcDiagnostic::error(format!("Invalid Character `{x0}`")).with_labels([span1.into()])
}

#[cold]
pub fn unterminated_multi_line_comment(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Unterminated multiline comment").with_labels([span0.into()])
}

#[cold]
pub fn unterminated_string(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Unterminated string").with_labels([span0.into()])
}

#[cold]
pub fn invalid_css_selector(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Invalid selector").with_labels([span0.into()])
}

#[cold]
pub fn unexpected_end(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Unexpected end of file").with_labels([span0.into()])
}
