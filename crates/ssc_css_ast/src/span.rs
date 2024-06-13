use oxc_span::{GetSpan, Span};

use crate::ast::*;

impl<'a> GetSpan for SimpleSelector<'a> {
    fn span(&self) -> Span {
        match self {
            Self::TypeSelector(selector) => selector.span,
            Self::IdSelector(selector) => selector.span,
            Self::ClassSelector(selector) => selector.span,
            Self::AttributeSelector(selector) => selector.span,
            Self::PseudoElementSelector(selector) => selector.span,
            Self::PseudoClassSelector(selector) => selector.span,
            Self::PercentageSelector(selector) => selector.span,
            Self::NthSelector(selector) => selector.span,
            Self::NestingSelector(selector) => selector.span,
        }
    }
}

impl<'a> GetSpan for BlockChild<'a> {
    fn span(&self) -> Span {
        match self {
            Self::Declaration(declaration) => declaration.span,
            Self::StyleRule(rule) => rule.span,
            Self::AtRule(rule) => rule.span,
        }
    }
}

impl<'a> GetSpan for Rule<'a> {
    fn span(&self) -> Span {
        match self {
            Self::StyleRule(rule) => rule.span,
            Self::AtRule(rule) => rule.span,
        }
    }
}
