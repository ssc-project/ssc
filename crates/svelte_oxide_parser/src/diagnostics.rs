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
pub fn unexpected_end(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Unexpected end of file").with_labels([span0.into()])
}

#[cold]
pub fn invalid_render_tag_expression(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("`{@render ...}` tags can only contain call expression")
        .with_labels([span0.into()])
}

#[cold]
pub fn invalid_modifier(span0: Span, name: &str, valid: &[&str]) -> OxcDiagnostic {
    OxcDiagnostic::error(format!(
        "Invalid modifier `{}`, valid modifiers are: {}",
        name,
        valid.join(", ")
    ))
    .with_labels([span0.into()])
}

#[cold]
pub fn duplicate_script(span0: Span, span1: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("A component can have a single top-level `<script>` element and/or a single top-level `<script context=\"module\">` element").with_labels([
        LabeledSpan::new_with_span(Some("First top-level script defined here".to_string()), span0),
        LabeledSpan::new_with_span(Some("It cannot be redefined here".to_string()), span1)
    ])
}

#[cold]
pub fn duplicate_style(span0: Span, span1: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("A component can have a single top-level `<style>` element").with_labels([
        LabeledSpan::new_with_span(
            Some("First top-level `<style>` element first defined here".to_string()),
            span0,
        ),
        LabeledSpan::new_with_span(Some("It cannot be redefined here".to_string()), span1),
    ])
}

#[cold]
pub fn missing_directive_name(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Missing directive name").with_labels([span0.into()])
}

#[cold]
pub fn invalid_directive_value(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Directive value must be a JavaScript expression enclosed in curly braces")
        .with_labels([span0.into()])
}

#[cold]
pub fn invalid_bind_directive_value(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Bind directive value must be an identifier or a member expression")
        .with_labels([span0.into()])
}

#[cold]
pub fn missing_class_directive_value(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Missing class directive value").with_labels([span0.into()])
}

#[cold]
pub fn invalid_let_directive_value(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error(
        "Let directive value must one of identifier, array expression, or object expression",
    )
    .with_labels([span0.into()])
}

#[cold]
pub fn unknown_directive_type(span0: Span, name: &str) -> OxcDiagnostic {
    OxcDiagnostic::error(format!("Unknown directive `{}`, valid directives are: `animate`, `bind`, `class`, `let`, `on`, `style`, `transition`, `in`, `out`, `use`", name)).with_labels([span0.into()])
}

#[cold]
pub fn svelte_component_missing_this(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("`<svelte:component>` must have a 'this' attribute")
        .with_labels([span0.into()])
}

#[cold]
pub fn svelte_component_invalid_this(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Invalid component definition - must be an `{expression}`")
        .with_labels([span0.into()])
}

#[cold]
pub fn svelte_element_missing_this(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("`<svelte:element>` must have a 'this' attribute")
        .with_labels([span0.into()])
}
