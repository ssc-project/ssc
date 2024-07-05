use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;
use ssc_css_ast::ast::CombinatorKind;

pub fn invalid_nesting_selector_placement(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("Nesting selectors can only be used inside a rule").with_label(span)
}

pub fn invalid_global_placement(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error(
        ":global(...) can be at the start or end of a selector sequence, but not in the middle",
    )
    .with_label(span)
}

pub fn invalid_global_selector_list(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error(":global(...) must not contain type or universal selectors when used in a compound selector").with_label(span)
}

pub fn invalid_type_selector_placement(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error(":global(...) must not be followed by a type selector").with_label(span)
}

pub fn invalid_global_selector(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error(":global(...) must contain exactly one selector").with_label(span)
}

pub fn invalid_global_block_list(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error(
        "A :global {...} block cannot be part of a selector list with more than one item",
    )
    .with_label(span)
}

pub fn invalid_global_block_modifier(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("A :global {...} block cannot modify an existing selector")
        .with_label(span)
}

pub fn invalid_global_block_combinator(span: Span, kind: CombinatorKind) -> OxcDiagnostic {
    OxcDiagnostic::error(format!(
        "A :global {{...}} block cannot follow a {} combinator",
        kind.as_str()
    ))
    .with_label(span)
}

pub fn invalid_global_block_declaration(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("A :global {...} block can only contain rules, not declarations")
        .with_label(span)
}
