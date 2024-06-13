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

        impl<'a> GetSpan for AstKind<'a> {
            #[allow(clippy::match_same_arms)]
            fn span(&self) -> Span {
                match self {
                    $(Self::$ident(x) => x.span),*
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
            pub fn debug_name(&self) -> &str {
                match self {
                    $(Self::$ident(_) => stringify!($ident)),*
                }
            }
        }
    )
}

ast_kinds! {
    StyleSheet(&'a StyleSheet<'a>),
    AtRule(&'a AtRule<'a>),
    StyleRule(&'a StyleRule<'a>),
    ComplexSelector(&'a ComplexSelector<'a>),
    RelativeSelector(&'a RelativeSelector<'a>),
    TypeSelector(&'a TypeSelector<'a>),
    IdSelector(&'a IdSelector<'a>),
    ClassSelector(&'a ClassSelector<'a>),
    AttributeSelector(&'a AttributeSelector<'a>),
    PseudoElementSelector(&'a PseudoElementSelector<'a>),
    PseudoClassSelector(&'a PseudoClassSelector<'a>),
    PercentageSelector(&'a PercentageSelector<'a>),
    NthSelector(&'a NthSelector<'a>),
    NestingSelector(&'a NestingSelector),
    Combinator(&'a Combinator),
    Block(&'a Block<'a>),
    Declaration(&'a Declaration<'a>),
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
