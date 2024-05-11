use oxc_allocator::Vec;
use oxc_span::{Atom, Span};
#[cfg(feature = "serialize")]
use serde::Serialize;

use super::macros::define_constant_string;
use crate::ast::{Attribute, Comment};

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct StyleSheet<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub attributes: Vec<'a, Attribute<'a>>,
    pub children: Vec<'a, AtruleOrRule<'a>>,
    pub content: StyleSheetContent<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct StyleSheetContent<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub styles: Atom<'a>,
    pub comment: Option<Comment<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum AtruleOrRule<'a> {
    Atrule(Atrule<'a>),
    Rule(Rule<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Atrule<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub prelude: Atom<'a>,
    pub block: Option<CssBlock<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Rule<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub prelude: SelectorList<'a>,
    pub block: CssBlock<'a>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: RuleMetadata,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct RuleMetadata {
    // TODO: add `parent_rule`
    pub has_local_selectors: bool,
    pub is_global_block: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SelectorList<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub children: Vec<'a, ComplexSelector<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct ComplexSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub children: Vec<'a, RelativeSelector<'a>>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: ComplexSelectorMetadata,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct ComplexSelectorMetadata {
    // TODO: add `rule`
    pub used: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct RelativeSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub combinator: Option<Combinator<'a>>,
    pub selectors: Vec<'a, SimpleSelector<'a>>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: RelativeSelectorMetadata,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct TypeSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct IdSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct ClassSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct AttributeSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub matcher: Option<Atom<'a>>,
    pub value: Option<Atom<'a>>,
    pub flags: Option<Atom<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct PseudoElementSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct PseudoClassSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub args: Option<SelectorList<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type", rename = "Percentage"))]
pub struct PercentageSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub value: Atom<'a>,
}

define_constant_string!(NestingSelectorName => "&");

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct NestingSelector {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: NestingSelectorName,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type", rename = "Nth"))]
pub struct NthSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub value: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum SimpleSelector<'a> {
    TypeSelector(TypeSelector<'a>),
    IdSelector(IdSelector<'a>),
    ClassSelector(ClassSelector<'a>),
    AttributeSelector(AttributeSelector<'a>),
    PseudoElementSelector(PseudoElementSelector<'a>),
    PseudoClassSelector(PseudoClassSelector<'a>),
    PercentageSelector(PercentageSelector<'a>),
    NthSelector(NthSelector<'a>),
    NestingSelector(NestingSelector),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct RelativeSelectorMetadata {
    pub is_global: bool,
    pub is_host: bool,
    pub root: bool,
    pub scoped: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Combinator<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct CssBlock<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub children: Vec<'a, BlockChild<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub enum BlockChild<'a> {
    Declaration(Declaration<'a>),
    Rule(Rule<'a>),
    Atrule(Atrule<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Declaration<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub property: Atom<'a>,
    pub value: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum CssNode<'a> {
    StyleSheet(StyleSheet<'a>),
    Rule(Rule<'a>),
    Atrule(Atrule<'a>),
    SelectorList(SelectorList<'a>),
    Block(CssBlock<'a>),
    ComplexSelector(ComplexSelector<'a>),
    RelativeSelector(RelativeSelector<'a>),
    Combinator(Combinator<'a>),
    SimpleSelector(SimpleSelector<'a>),
    Declaration(Declaration<'a>),
}
