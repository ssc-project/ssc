use oxc_allocator::Vec;
use oxc_span::{Atom, Span};
#[cfg(feature = "serialize")]
use serde::Serialize;

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct StyleSheet<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub children: Vec<'a, Rule<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Rule<'a> {
    AtRule(AtRule<'a>),
    StyleRule(StyleRule<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type", rename = "Atrule"))]
pub struct AtRule<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub prelude: Atom<'a>,
    pub block: Option<Block<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type", rename = "Rule"))]
pub struct StyleRule<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub prelude: SelectorList<'a>,
    pub block: Block<'a>,
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
    pub combinator: Option<Combinator>,
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
    pub matcher: Option<AttributeMatcher>,
    pub value: Option<Atom<'a>>,
    pub flags: Option<Atom<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum AttributeMatcher {
    #[cfg_attr(feature = "serialize", serde(rename = "~="))]
    /// `~=`
    Substring,
    #[cfg_attr(feature = "serialize", serde(rename = "^="))]
    /// `^=`
    Prefix,
    #[cfg_attr(feature = "serialize", serde(rename = "$="))]
    /// `$=`
    Suffix,
    #[cfg_attr(feature = "serialize", serde(rename = "*="))]
    /// `*=`
    Includes,
    #[cfg_attr(feature = "serialize", serde(rename = "|="))]
    /// `|=`
    DashMatch,
    #[cfg_attr(feature = "serialize", serde(rename = "="))]
    /// `=`
    Equal,
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
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct NestingSelector {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
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
pub struct Combinator {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    #[cfg_attr(feature = "serialize", serde(rename = "name"))]
    pub kind: CombinatorKind,
}

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum CombinatorKind {
    #[cfg_attr(feature = "serialize", serde(rename = "+"))]
    NextSibling,
    #[cfg_attr(feature = "serialize", serde(rename = "~"))]
    LaterSibling,
    #[cfg_attr(feature = "serialize", serde(rename = ">"))]
    Child,
    #[cfg_attr(feature = "serialize", serde(rename = "||"))]
    Column,
    #[cfg_attr(feature = "serialize", serde(rename = " "))]
    Descendant,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Block<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub children: Vec<'a, BlockChild<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub enum BlockChild<'a> {
    Declaration(Declaration<'a>),
    StyleRule(StyleRule<'a>),
    AtRule(AtRule<'a>),
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
pub enum Node<'a> {
    StyleSheet(StyleSheet<'a>),
    Rule(StyleRule<'a>),
    Atrule(AtRule<'a>),
    SelectorList(SelectorList<'a>),
    Block(Block<'a>),
    ComplexSelector(ComplexSelector<'a>),
    RelativeSelector(RelativeSelector<'a>),
    Combinator(Combinator),
    SimpleSelector(SimpleSelector<'a>),
    Declaration(Rule<'a>),
}
