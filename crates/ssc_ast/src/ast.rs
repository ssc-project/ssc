// Silence erroneous warnings from Rust Analyser for `#[derive(Tsify)]`
#![allow(non_snake_case)]

use oxc_allocator::Vec;
use oxc_ast::ast::{
    ArrayExpression, ArrowFunctionExpression, BindingPattern, CallExpression, Expression,
    IdentifierName, IdentifierReference, MemberExpression, ObjectExpression, Program,
    VariableDeclaration,
};
use oxc_span::{Atom, Span};
use rustc_hash::FxHashMap;
#[cfg(feature = "serialize")]
use serde::Serialize;
use ssc_css_ast::ast::StyleSheet;
#[cfg(feature = "serialize")]
use tsify::Tsify;

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Root<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub options: Option<SvelteOptions<'a>>,
    pub fragment: Fragment<'a>,
    pub css: Option<Style<'a>>,
    pub instance: Option<Script<'a>>,
    pub module: Option<Script<'a>>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: RootMetadata,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Fragment<'a> {
    pub nodes: Vec<'a, FragmentNode<'a>>,
    pub transparent: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum FragmentNode<'a> {
    Text(Text<'a>),
    Tag(Tag<'a>),
    Element(Element<'a>),
    Block(Block<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Text<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub data: Atom<'a>,
    pub raw: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Tag<'a> {
    ExpressionTag(ExpressionTag<'a>),
    HtmlTag(HtmlTag<'a>),
    ConstTag(ConstTag<'a>),
    DebugTag(DebugTag<'a>),
    RenderTag(RenderTag<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct ExpressionTag<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub expression: Expression<'a>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: ExpressionTagMetadata,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
pub struct ExpressionTagMetadata {
    pub contains_call_expression: bool,
    pub dynamic: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct HtmlTag<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub expression: Expression<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct ConstTag<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub declaration: VariableDeclaration<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct DebugTag<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub identifiers: Vec<'a, IdentifierReference<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct RenderTag<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub expression: RenderTagExpression<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum RenderTagExpression<'a> {
    Call(CallExpression<'a>),
    Chain(CallExpression<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Element<'a> {
    Component(Component<'a>),
    TitleElement(TitleElement<'a>),
    SlotElement(SlotElement<'a>),
    RegularElement(RegularElement<'a>),
    SvelteBody(SvelteBody<'a>),
    SvelteComponent(SvelteComponent<'a>),
    SvelteDocument(SvelteDocument<'a>),
    SvelteElement(SvelteElement<'a>),
    SvelteFragment(SvelteFragment<'a>),
    SvelteHead(SvelteHead<'a>),
    SvelteOptionsRaw(SvelteOptionsRaw<'a>),
    SvelteSelf(SvelteSelf<'a>),
    SvelteWindow(SvelteWindow<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum ElementAttribute<'a> {
    Attribute(Attribute<'a>),
    SpreadAttribute(SpreadAttribute<'a>),
    DirectiveAttribute(DirectiveAttribute<'a>),
}

impl<'a> ElementAttribute<'a> {
    pub fn as_attribute(&self) -> Option<&Attribute<'a>> {
        if let ElementAttribute::Attribute(attribute) = self {
            Some(attribute)
        } else {
            None
        }
    }

    pub fn as_spread_attribute(&self) -> Option<&SpreadAttribute<'a>> {
        if let ElementAttribute::SpreadAttribute(attribute) = self {
            Some(attribute)
        } else {
            None
        }
    }

    pub fn as_directive_attribute(&self) -> Option<&DirectiveAttribute<'a>> {
        if let ElementAttribute::DirectiveAttribute(directive) = self {
            Some(directive)
        } else {
            None
        }
    }

    pub fn attribute(self) -> Option<Attribute<'a>> {
        if let ElementAttribute::Attribute(attribute) = self {
            Some(attribute)
        } else {
            None
        }
    }

    pub fn spread_attribute(self) -> Option<SpreadAttribute<'a>> {
        if let ElementAttribute::SpreadAttribute(attribute) = self {
            Some(attribute)
        } else {
            None
        }
    }

    pub fn directive_attribute(self) -> Option<DirectiveAttribute<'a>> {
        if let ElementAttribute::DirectiveAttribute(directive) = self {
            Some(directive)
        } else {
            None
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Component<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct TitleElement<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SlotElement<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct RegularElement<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: RegularElementMetadata,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
pub struct RegularElementMetadata {
    pub svg: bool,
    pub has_spread: bool,
    pub scoped: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteBody<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteComponent<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
    pub expression: Expression<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteDocument<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteElement<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
    pub expression: Expression<'a>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: SvelteElementMetadata,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
pub struct SvelteElementMetadata {
    pub svg: bool,
    pub scoped: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteFragment<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteHead<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteOptionsRaw<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteSelf<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteWindow<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Block<'a> {
    EachBlock(EachBlock<'a>),
    IfBlock(IfBlock<'a>),
    AwaitBlock(AwaitBlock<'a>),
    KeyBlock(KeyBlock<'a>),
    SnippetBlock(SnippetBlock<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct EachBlock<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub expression: Expression<'a>,
    pub context: BindingPattern<'a>,
    pub body: Fragment<'a>,
    pub fallback: Option<Fragment<'a>>,
    // Difference from the original svelte compiler, the original svelte
    // compiler uses `String` instead of `IdentifierName`
    pub index: Option<IdentifierName<'a>>,
    pub key: Option<Expression<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct IfBlock<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub elseif: bool,
    pub test: Expression<'a>,
    pub consequent: Fragment<'a>,
    pub alternate: Option<Fragment<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct AwaitBlock<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub expression: Expression<'a>,
    pub value: Option<BindingPattern<'a>>,
    pub error: Option<BindingPattern<'a>>,
    pub pending: Option<Fragment<'a>>,
    pub then: Option<Fragment<'a>>,
    pub catch: Option<Fragment<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct KeyBlock<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub expression: Expression<'a>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SnippetBlock<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub expression: IdentifierName<'a>,
    pub parameters: Vec<'a, BindingPattern<'a>>,
    pub body: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(rename = "Style", tag = "type"))]
pub struct Style<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub attributes: Vec<'a, Attribute<'a>>,
    pub stylesheet: StyleSheet<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct SvelteOptions<'a> {
    pub span: Span,
    pub runes: Option<bool>,
    pub immutable: Option<bool>,
    pub accessors: Option<bool>,
    pub preserve_whitespace: Option<bool>,
    pub namespace: Option<Namespace>,
    pub custom_element: Option<CustomElementOptions<'a>>,
    pub attributes: Vec<'a, Attribute<'a>>,
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Namespace {
    #[default]
    Html,
    Svg,
    MathMl,
    Foreign,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
pub struct CustomElementOptions<'a> {
    pub tag: Atom<'a>,
    #[cfg_attr(feature = "serialize", tsify(type = r#""open" | "none" | null"#))]
    pub shadow: Option<CustomElementShadow>,
    #[cfg_attr(feature = "serialize", tsify(type = r#"Map<Atom, CustomElementProp>"#))]
    pub props: FxHashMap<Atom<'a>, CustomElementProp<'a>>,
    pub extend: Option<CustomElementExtend<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum CustomElementShadow {
    Open,
    None,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
pub struct CustomElementProp<'a> {
    pub attribute: Option<Atom<'a>>,
    pub reflect: Option<bool>,
    #[cfg_attr(feature = "serialize", serde(rename = "type"))]
    #[cfg_attr(
        feature = "serialize",
        tsify(type = r#""Array" | "Boolean" | "Number" | "Object" | "String""#)
    )]
    pub type_: Option<CustomElementPropType>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub enum CustomElementPropType {
    Array,
    Boolean,
    Number,
    Object,
    String,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum CustomElementExtend<'a> {
    ArrowFunction(ArrowFunctionExpression<'a>),
    Identifier(IdentifierReference<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Script<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    #[cfg_attr(feature = "serialize", tsify(type = r#""default" | "module""#))]
    pub context: ScriptContext,
    #[cfg_attr(feature = "serialize", serde(rename = "content"))]
    pub program: Program<'a>,
    pub attributes: Vec<'a, Attribute<'a>>,
}

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum ScriptContext {
    Default,
    Module,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct RootMetadata {
    pub ts: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Attribute<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub value: AttributeValue<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum AttributeValue<'a> {
    Bool(bool), // true
    Sequence(Vec<'a, AttributeSequenceValue<'a>>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum AttributeSequenceValue<'a> {
    Text(Text<'a>),
    ExpressionTag(ExpressionTag<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SpreadAttribute<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub expression: Expression<'a>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: SpreadAttributeMetadata,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct SpreadAttributeMetadata {
    pub contains_call_expression: bool,
    pub dynamic: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum DirectiveAttribute<'a> {
    AnimateDirective(AnimateDirective<'a>),
    BindDirective(BindDirective<'a>),
    ClassDirective(ClassDirective<'a>),
    LetDirective(LetDirective<'a>),
    OnDirective(OnDirective<'a>),
    StyleDirective(StyleDirective<'a>),
    TransitionDirective(TransitionDirective<'a>),
    UseDirective(UseDirective<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct AnimateDirective<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub expression: Option<Expression<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct BindDirective<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub expression: BindDirectiveExpression<'a>,
    // TODO: figure this out
    // pub metadata: BindDirectiveMetadata<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum BindDirectiveExpression<'a> {
    Identifier(IdentifierReference<'a>),
    MemberExpression(MemberExpression<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct BindDirectiveMetadata<'a> {
    pub binding_group_name: IdentifierReference<'a>,
    pub parent_each_blocks: Vec<'a, &'a EachBlock<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct ClassDirective<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub expression: Expression<'a>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: ClassDirectiveMetadata,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct ClassDirectiveMetadata {
    pub dynamic: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct LetDirective<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub expression: Option<LetDirectiveExpression<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum LetDirectiveExpression<'a> {
    Identifier(IdentifierReference<'a>),
    ArrayExpression(ArrayExpression<'a>),
    ObjectExpression(ObjectExpression<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct OnDirective<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub expression: Option<Expression<'a>>,
    // TODO: use concrete type instead of Atom
    pub modifiers: Vec<'a, Atom<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct StyleDirective<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub value: AttributeValue<'a>,
    pub modifiers: Vec<'a, StyleDirectiveModifier>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: StyleDirectiveMetadata,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum StyleDirectiveModifier {
    Important,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct StyleDirectiveMetadata {
    pub dynamic: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct TransitionDirective<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub expression: Option<Expression<'a>>,
    pub modifiers: Vec<'a, TransitionDirectiveModifier>,
    pub intro: bool,
    pub outro: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum TransitionDirectiveModifier {
    Local,
    Global,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct UseDirective<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub expression: Option<Expression<'a>>,
}
