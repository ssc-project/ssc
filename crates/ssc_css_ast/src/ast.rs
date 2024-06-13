// Silence erroneous warnings from Rust Analyser for `#[derive(Tsify)]`
#![allow(non_snake_case)]

use std::cell::Cell;

use bitflags::bitflags;
use oxc_allocator::Vec;
use oxc_index::define_index_type;
use oxc_span::{Atom, Span};
#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "serialize")]
use tsify::Tsify;

#[cfg(feature = "serialize")]
#[wasm_bindgen::prelude::wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type AstNodeId = number;
export type RuleFlags = {
    GlobalBlock: 1,
    LocalSelectors: 2,
};
export type RelativeSelectorFlags = {
    Global: 1,
    GlobalLike: 2,
    Host: 4,
    Root: 8,
    Scoped: 16,
};
"#;

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct StyleSheet<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub children: Vec<'a, Rule<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Rule<'a> {
    AtRule(AtRule<'a>),
    StyleRule(StyleRule<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type", rename = "Atrule"))]
pub struct AtRule<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub prelude: Atom<'a>,
    pub block: Option<Block<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type", rename = "Rule"))]
pub struct StyleRule<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub prelude: SelectorList<'a>,
    pub block: Block<'a>,
    #[cfg_attr(feature = "serialize", serde(skip))]
    pub parent_rule: Cell<Option<AstNodeId>>,
    #[cfg_attr(feature = "serialize", serde(skip))]
    pub flags: Cell<RuleFlags>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SelectorList<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub children: Vec<'a, ComplexSelector<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct ComplexSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub children: Vec<'a, RelativeSelector<'a>>,
    #[cfg_attr(feature = "serialize", serde(skip))]
    pub rule: Cell<Option<AstNodeId>>,
    pub used: Cell<bool>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct RelativeSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub combinator: Option<Combinator>,
    pub selectors: Vec<'a, SimpleSelector<'a>>,
    #[cfg_attr(feature = "serialize", serde(skip))]
    pub flags: Cell<RelativeSelectorFlags>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct TypeSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct IdSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct ClassSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
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
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
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
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct PseudoElementSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct PseudoClassSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub args: Option<SelectorList<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type", rename = "Percentage"))]
pub struct PercentageSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub value: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type", rename = "Nth"))]
pub struct NthSelector<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub value: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct NestingSelector {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
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

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Combinator {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    #[cfg_attr(feature = "serialize", serde(rename = "name"))]
    pub kind: CombinatorKind,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
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

impl CombinatorKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::NextSibling => "+",
            Self::LaterSibling => "~",
            Self::Child => ">",
            Self::Column => "||",
            Self::Descendant => " ",
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Block<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub children: Vec<'a, BlockChild<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub enum BlockChild<'a> {
    Declaration(Declaration<'a>),
    StyleRule(StyleRule<'a>),
    AtRule(AtRule<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Declaration<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub property: Atom<'a>,
    pub value: Atom<'a>,
}

define_index_type! {
    pub struct AstNodeId = usize;
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RuleFlags: u8 {
        const GlobalBlock    = 1 << 0;
        const LocalSelectors = 1 << 1;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RelativeSelectorFlags: u8 {
        const Global     = 1 << 0;
        const GlobalLike = 1 << 1;
        const Host       = 1 << 2;
        const Root       = 1 << 3;
        const Scoped     = 1 << 4;
    }
}

impl RuleFlags {
    #[inline]
    pub fn has_global_block(&self) -> bool {
        self.contains(Self::GlobalBlock)
    }

    #[inline]
    pub fn has_local_selectors(&self) -> bool {
        self.contains(Self::LocalSelectors)
    }
}

impl RelativeSelectorFlags {
    #[inline]
    pub fn has_global(&self) -> bool {
        self.contains(Self::Global)
    }

    pub fn has_global_like(&self) -> bool {
        self.contains(Self::GlobalLike)
    }

    #[inline]
    pub fn has_host(&self) -> bool {
        self.contains(Self::Host)
    }

    #[inline]
    pub fn has_root(&self) -> bool {
        self.contains(Self::Root)
    }

    #[inline]
    pub fn has_scoped(&self) -> bool {
        self.contains(Self::Scoped)
    }
}
