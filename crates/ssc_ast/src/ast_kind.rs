use oxc_span::{GetSpan, Span};

#[allow(clippy::wildcard_imports)]
use crate::ast::*;

macro_rules! ast_kinds {
    { $($ident:ident($type:ty),)* } => (
        #[derive(Debug, Clone, Copy)]
        pub enum AstType {
            $($ident,)*
        }

        /// Untyped AST Node Kind
        #[derive(Debug, Clone, Copy)]
        pub enum AstKind<'a> {
            $($ident($type),)*
        }
    )
}

ast_kinds! {
    Root(&'a Root<'a>),
    Text(&'a Text<'a>),
    ExpressionTag(&'a ExpressionTag<'a>),
    HtmlTag(&'a HtmlTag<'a>),
    ConstTag(&'a ConstTag<'a>),
    DebugTag(&'a DebugTag<'a>),
    RenderTag(&'a RenderTag<'a>),
    Component(&'a Component<'a>),
    TitleElement(&'a TitleElement<'a>),
    SlotElement(&'a SlotElement<'a>),
    RegularElement(&'a RegularElement<'a>),
    SvelteBody(&'a SvelteBody<'a>),
    SvelteComponent(&'a SvelteComponent<'a>),
    SvelteDocument(&'a SvelteDocument<'a>),
    SvelteElement(&'a SvelteElement<'a>),
    SvelteFragment(&'a SvelteFragment<'a>),
    SvelteHead(&'a SvelteHead<'a>),
    SvelteOptionsRaw(&'a SvelteOptionsRaw<'a>),
    SvelteSelf(&'a SvelteSelf<'a>),
    SvelteWindow(&'a SvelteWindow<'a>),
    EachBlock(&'a EachBlock<'a>),
    IfBlock(&'a IfBlock<'a>),
    AwaitBlock(&'a AwaitBlock<'a>),
    KeyBlock(&'a KeyBlock<'a>),
    SnippetBlock(&'a SnippetBlock<'a>),
}

#[allow(unsafe_code)]
// SAFETY:
// The AST is part of the bump allocator,
// it is our responsibility to never simultaneously mutate across threads.
unsafe impl<'a> Send for AstKind<'a> {}
#[allow(unsafe_code)]
// SAFETY:
// The AST is part of the bump allocator,
// it is our responsibility to never simultaneously mutate across threads.
unsafe impl<'a> Sync for AstKind<'a> {}

impl<'a> GetSpan for AstKind<'a> {
    #[allow(clippy::match_same_arms)]
    fn span(&self) -> Span {
        match self {
            Self::Root(x) => x.span,
            Self::Text(x) => x.span,
            Self::ExpressionTag(x) => x.span,
            Self::HtmlTag(x) => x.span,
            Self::ConstTag(x) => x.span,
            Self::DebugTag(x) => x.span,
            Self::RenderTag(x) => x.span,
            Self::Component(x) => x.span,
            Self::TitleElement(x) => x.span,
            Self::SlotElement(x) => x.span,
            Self::RegularElement(x) => x.span,
            Self::SvelteBody(x) => x.span,
            Self::SvelteComponent(x) => x.span,
            Self::SvelteDocument(x) => x.span,
            Self::SvelteElement(x) => x.span,
            Self::SvelteFragment(x) => x.span,
            Self::SvelteHead(x) => x.span,
            Self::SvelteOptionsRaw(x) => x.span,
            Self::SvelteSelf(x) => x.span,
            Self::SvelteWindow(x) => x.span,
            Self::EachBlock(x) => x.span,
            Self::IfBlock(x) => x.span,
            Self::AwaitBlock(x) => x.span,
            Self::KeyBlock(x) => x.span,
            Self::SnippetBlock(x) => x.span,
        }
    }
}

impl<'a> AstKind<'a> {
    #[allow(clippy::match_same_arms)]
    /// Get the AST kind name with minimal details. Particularly useful for
    /// when debugging an iteration over an AST.
    ///
    /// Note that this method does not exist in release builds. Do not include
    /// usage of this method within your code.
    pub fn debug_name(&self) -> std::borrow::Cow<str> {
        match self {
            Self::Root(_) => "Root".into(),
            Self::Text(_) => "Text".into(),
            Self::ExpressionTag(_) => "ExpressionTag".into(),
            Self::HtmlTag(_) => "HtmlTag".into(),
            Self::ConstTag(_) => "ConstTag".into(),
            Self::DebugTag(_) => "DebugTag".into(),
            Self::RenderTag(_) => "RenderTag".into(),
            Self::Component(_) => "Component".into(),
            Self::TitleElement(_) => "TitleElement".into(),
            Self::SlotElement(_) => "SlotElement".into(),
            Self::RegularElement(_) => "RegularElement".into(),
            Self::SvelteBody(_) => "SvelteBody".into(),
            Self::SvelteComponent(_) => "SvelteComponent".into(),
            Self::SvelteDocument(_) => "SvelteDocument".into(),
            Self::SvelteElement(_) => "SvelteElement".into(),
            Self::SvelteFragment(_) => "SvelteFragment".into(),
            Self::SvelteHead(_) => "SvelteHead".into(),
            Self::SvelteOptionsRaw(_) => "SvelteOptionsRaw".into(),
            Self::SvelteSelf(_) => "SvelteSelf".into(),
            Self::SvelteWindow(_) => "SvelteWindow".into(),
            Self::EachBlock(_) => "EachBlock".into(),
            Self::IfBlock(_) => "IfBlock".into(),
            Self::AwaitBlock(_) => "AwaitBlock".into(),
            Self::KeyBlock(_) => "KeyBlock".into(),
            Self::SnippetBlock(_) => "SnippetBlock".into(),
        }
    }
}
