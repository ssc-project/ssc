pub mod internal {
    use oxc_diagnostics::{
        miette::{self, Diagnostic},
        thiserror::{self, Error},
    };
    use oxc_span::Span;

    #[derive(Debug, Error, Diagnostic)]
    #[error("TODO {1}")]
    pub struct Todo(#[label] pub Span, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Internal compiler error: {1}. Please report this to https://github.com/a-rustacean/svelte-compiler/issues")]
    pub struct Internal(#[label] pub Span, pub String);
}

pub mod parse {
    use oxc_diagnostics::{
        miette::{self, Diagnostic},
        thiserror::{self, Error},
    };
    use oxc_span::Span;

    #[derive(Debug, Error, Diagnostic)]
    #[error("<{1}> was left open")]
    pub struct UnclosedElement(#[label] pub Span, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Block was left open")]
    pub struct UnclosedBlock(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Unexpected block closing tag")]
    pub struct UnexpectedBlockClose(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Unexpected end of input")]
    pub struct UnexpectedEof(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Unexpected end of input (expected {1})")]
    pub struct UnexpectedEofWithExpected(#[label] pub Span, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error("{1}")]
    pub struct JsParseError(#[label] pub Span, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Expected token {1}")]
    pub struct UnexpectedToken(#[label] pub Span, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error("'{1}' is a reserved word in javascript and cannot be used here")]
    pub struct UnexpectedReservedWord(#[label] pub Span, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Expected whitespace")]
    pub struct MissingWhitespace(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Expected identifier or destructure pattern")]
    pub struct ExpectedPattern(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error(
        "If the context attribute is supplied its value must be \"module\""
    )]
    pub struct InvalidScriptContext(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("'elseif' should be 'else if'")]
    pub struct InvalidElseif(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error(
        "{{:...}} block is invalid at this position (did you forget to close \
         the preceeding element or block?)"
    )]
    pub struct InvalidContinuingBlockPlacement(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("{1} block must be a child of {2}")] //     child,  parent
    pub struct InvalidBlockMissingParent(
        #[label] pub Span,
        pub String,
        pub String,
    );

    #[derive(Debug, Error, Diagnostic)]
    #[error("{1} cannot appear more than once within a block")]
    pub struct DuplicateBlockPart(#[label] pub Span, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Expected 'if', 'each', 'await', 'key' or 'snippet'")]
    pub struct ExpectedBlockType(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Expected an identifier")]
    pub struct ExpectedIdentifier(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error(
        "{{@debug ...}} arguments must be identifiers, not arbitrary \
         expressions"
    )]
    pub struct InvalidDebug(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("{{@const ...}} must be an assignment")]
    pub struct InvalidConst(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("{{#{2} ...}} block cannot be {1}")]
    pub struct InvalidBlockPlacement(#[label] pub Span, pub String, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error("{{@{2} ...}} tag cannot be {1}")]
    pub struct InvalidTagPlacement(#[label] pub Span, pub String, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Expected attribute value")]
    pub struct MissingAttributeValue(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Expected closing {1} character")]
    pub struct UnclosedAttributeValue(#[label] pub Span, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error(
        "Directive value must be a JavaScript expression enclosed in curly \
         braces"
    )]
    pub struct InvalidDirectiveValue(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("{1} name cannot be empty")]
    pub struct EmptyDirectiveName(#[label] pub Span, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error("</{1}> attempted to close an element that was not open")]
    pub struct InvalidClosingTag(#[label] pub Span, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error(
        "</{1}> attempted to close element that was already automactically \
         closed by <{2}>"
    )]
    pub struct InvalidClosingTagAfterAutoclose(
        #[label] pub Span,
        pub String,
        pub String,
    );

    #[derive(Debug, Error, Diagnostic)]
    #[error(
        "The $ name is reserved, and cannot be used for variables and imports"
    )]
    pub struct InvalidDollarBinding(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error(
        "The $ prefix is reserved, and cannot be used for vaiables and imports"
    )]
    pub struct InvalidDollarPrefix(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error(
        "The $ name is reserved. To reference a global variable called $m use \
         globalThis.$"
    )]
    pub struct InvalidDollarGlobal(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Cannot reference store value inside <script context=\"module\">")]
    pub struct IllegalSubscription(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("A component can have a single top-level <style> element")]
    pub struct DuplicateStyleElement(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error(
        "A component can have a single top-level <script> element and/or a \
         single top-level <script context=\"module\"> element"
    )]
    pub struct DuplicateScriptElement(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("{{@render ...}} tags can only contain call expression")]
    pub struct InvalidRenderExpression(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("exoected at most one argument")]
    pub struct InvalidRenderArguments(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error(
        "Calling a snippet function using apply, bind or call is not allowed"
    )]
    pub struct InvalidRenderCall(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("cannot use spread arguments in {{@render ...}} tags")]
    pub struct InvalidRenderSpreadArgument(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("snippets do not support rest parameters; use an array instead")]
    pub struct InvalidSnippetRestParamenter(#[label] pub Span);
}

pub mod special_elements {
    use oxc_diagnostics::{
        miette::{self, Diagnostic},
        thiserror::{self, Error},
    };
    use oxc_span::Span;

    #[derive(Debug, Error, Diagnostic)]
    #[error("<{1}> tags cannot be inside elements or blocks")]
    pub struct InvalidSvelteElementPlacement(#[label] pub Span, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error("A component can only have one <{1}> element")]
    pub struct DuplicateSvelteElement(#[label] pub Span, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error(
        "<svelte:self> component can only exist inside {{#if}} blocks, \
         {{#each}} blocks, {{#snippet}} blocks or slots passed to components"
    )]
    pub struct InvalidSelfPlacement(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("<svelte:element> must have a 'this' attribute")]
    pub struct MissingSvelteElementDefinition(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("<svelte:component> must have a 'this' attribute")]
    pub struct MissingSvelteComponentDefinition(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Invalid element definition - must be an {{expression}}")]
    pub struct InvalidSvelteElementDefinition(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Invalid component definition - must be an {{expression}}")]
    pub struct InvalidSvelteComponentDefinition(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Valid <svelte:...> tag names are {1}")]
    pub struct InvalidSvelteTag(#[label] pub Span, pub String);
}

pub mod elements {
    use oxc_diagnostics::{
        miette::{self, Diagnostic},
        thiserror::{self, Error},
    };
    use oxc_span::Span;

    #[derive(Debug, Error, Diagnostic)]
    #[error("<{1}> cannot have children")]
    pub struct InvalidElementContent(#[label] pub Span, pub String);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Expected valid tag name")]
    pub struct InvalidTagName(#[label] pub Span);
}

pub mod attributes {
    use oxc_diagnostics::{
        miette::{self, Diagnostic},
        thiserror::Error,
    };
    use oxc_span::Span;

    #[derive(Debug, Error, Diagnostic)]
    #[error("Attribute shorthand cannot be empty")]
    pub struct EmptyAttributeShorthand(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Attributes need to be unique")]
    pub struct DuplicateAttribute(#[label] pub Span);
}

pub mod css {
    use oxc_diagnostics::{
        miette::{self, Diagnostic},
        thiserror::Error,
    };
    use oxc_span::Span;

    #[derive(Debug, Error, Diagnostic)]
    #[error("Invalid selector")]
    pub struct InvalidCssSelector(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Expected a valid CSS identifier")]
    pub struct InvalidCssIdentifier(#[label] pub Span);

    #[derive(Debug, Error, Diagnostic)]
    #[error("Declaration cannot be empty")]
    pub struct InvalidCssDeclaration(#[label] pub Span);
}
