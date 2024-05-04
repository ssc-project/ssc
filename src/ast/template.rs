use oxc_allocator::{Allocator, Vec};
use oxc_ast::ast::{
    ArrayExpression, ArrowFunctionExpression, BigIntLiteral, BindingPattern,
    BooleanLiteral, CallExpression, CatchClause, Class, ClassBody,
    ExportSpecifier, Expression, Function, IdentifierName,
    ImportDefaultSpecifier, ImportNamespaceSpecifier, ImportSpecifier,
    MemberExpression, MethodDefinition, ModuleDeclaration, NullLiteral,
    NumericLiteral, ObjectExpression, ObjectProperty, PrivateIdentifier,
    Program, PropertyDefinition, RegExpLiteral, SpreadElement, Statement,
    StringLiteral, Super, SwitchCase, TemplateElement, VariableDeclaration,
    VariableDeclarator,
};
use oxc_span::{Atom, GetSpan, Span};
use rustc_hash::FxHashMap;
#[cfg(feature = "serialize")]
use serde::Serialize;

use super::{
    css::{Node as CssNode, StyleSheet},
    Binding,
};
use crate::define_constant_string;

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Fragment<'a> {
    pub nodes: Vec<'a, FragmentNodeKind<'a>>,
    pub transparent: bool,
}

impl<'a> Fragment<'a> {
    pub fn new(allocator: &'a Allocator, transparent: bool) -> Self {
        Self { nodes: Vec::new_in(allocator), transparent }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum FragmentNodeKind<'a> {
    Text(Text<'a>),
    Tag(Tag<'a>),
    ElementLike(ElementLike<'a>),
    Block(Block<'a>),
    Comment(Comment<'a>),
}

impl<'a> GetSpan for FragmentNodeKind<'a> {
    fn span(&self) -> Span {
        match self {
            FragmentNodeKind::Text(text) => text.span,
            FragmentNodeKind::Tag(tag) => tag.span(),
            FragmentNodeKind::ElementLike(element_like) => element_like.span(),
            FragmentNodeKind::Block(block) => block.span(),
            FragmentNodeKind::Comment(comment) => comment.span,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Text<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub data: Atom<'a>,
    pub raw: Atom<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Tag<'a> {
    Expression(ExpressionTag<'a>),
    Html(HtmlTag<'a>),
    Const(ConstTag<'a>),
    Debug(DebugTag<'a>),
    Render(RenderTag<'a>),
}

impl<'a> GetSpan for Tag<'a> {
    fn span(&self) -> Span {
        match self {
            Tag::Expression(expression) => expression.span,
            Tag::Html(html) => html.span,
            Tag::Const(const_tag) => const_tag.span,
            Tag::Debug(debug) => debug.span,
            Tag::Render(render) => render.span,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct ExpressionTag<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub expression: Expression<'a>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: ExpressionTagMetadata,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct ExpressionTagMetadata {
    pub contains_call_expression: bool,
    pub dynamic: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct HtmlTag<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub expression: Expression<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct ConstTag<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub declaration: VariableDeclaration<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct DebugTag<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub identifiers: Vec<'a, IdentifierName<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct RenderTag<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub expression: RenderTagExpression<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum RenderTagExpression<'a> {
    Call(CallExpression<'a>),
    Chain(CallExpression<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum ElementLike<'a> {
    Component(Component<'a>),
    Title(TitleElement<'a>),
    Slot(SlotElement<'a>),
    Regular(RegularElement<'a>),
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

impl<'a> GetSpan for ElementLike<'a> {
    fn span(&self) -> Span {
        match self {
            ElementLike::Component(component) => component.span,
            ElementLike::Title(title) => title.span,
            ElementLike::Slot(slot) => slot.span,
            ElementLike::Regular(regular) => regular.span,
            ElementLike::SvelteBody(svelte_body) => svelte_body.span,
            ElementLike::SvelteComponent(svelte_component) => {
                svelte_component.span
            }
            ElementLike::SvelteDocument(svelte_document) => {
                svelte_document.span
            }
            ElementLike::SvelteElement(svelte_element) => svelte_element.span,
            ElementLike::SvelteFragment(svelte_fragment) => {
                svelte_fragment.span
            }
            ElementLike::SvelteHead(svelte_head) => svelte_head.span,
            ElementLike::SvelteOptionsRaw(svelte_options_raw) => {
                svelte_options_raw.span
            }
            ElementLike::SvelteSelf(svelte_self) => svelte_self.span,
            ElementLike::SvelteWindow(svelte_window) => svelte_window.span,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum ElementAttribute<'a> {
    Attribute(Attribute<'a>),
    SpreadAttribute(SpreadAttribute<'a>),
    Directive(Directive<'a>),
}

impl<'a> GetSpan for ElementAttribute<'a> {
    fn span(&self) -> Span {
        match self {
            ElementAttribute::Attribute(attribute) => attribute.span,
            ElementAttribute::Directive(directive) => directive.span(),
            ElementAttribute::SpreadAttribute(spread_attribute) => {
                spread_attribute.span
            }
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Component<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

define_constant_string!(TitleElementName => "title");
define_constant_string!(SlotElementName => "slot");
define_constant_string!(SvelteBodyName => "svelte:body");
define_constant_string!(SvelteComponentName => "svelte:component");
define_constant_string!(SvelteDocumentName => "svelte:document");
define_constant_string!(SvelteElementName => "svelte:element");
define_constant_string!(SvelteFragmentName => "svelte:fragment");
define_constant_string!(SvelteHeadName => "svelte:head");
define_constant_string!(SvelteOptionsRawName => "svelte:options");
define_constant_string!(SvelteSelfName => "svelte:self");
define_constant_string!(SvelteWindowName => "svelte:window");

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct TitleElement<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: TitleElementName,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SlotElement<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: SlotElementName,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct RegularElementMetadata {
    pub svg: bool,
    pub has_spread: bool,
    pub scoped: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteBody<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: SvelteBodyName,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteComponent<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: SvelteComponentName,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
    pub expression: Expression<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteDocument<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: SvelteDocumentName,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteElement<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: SvelteElementName,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
    pub expression: Expression<'a>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: SvelteElementMetadata,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct SvelteElementMetadata {
    pub svg: bool,
    pub scoped: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteFragment<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: SvelteFragmentName,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteHead<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: SvelteHeadName,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteOptionsRaw<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: SvelteOptionsRawName,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteSelf<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: SvelteSelfName,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SvelteWindow<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: SvelteWindowName,
    pub attributes: Vec<'a, ElementAttribute<'a>>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Block<'a> {
    Each(EachBlock<'a>),
    If(IfBlock<'a>),
    Await(AwaitBlock<'a>),
    Key(KeyBlock<'a>),
    Snippet(SnippetBlock<'a>),
}

impl<'a> GetSpan for Block<'a> {
    fn span(&self) -> Span {
        match self {
            Block::Each(each) => each.span,
            Block::If(if_block) => if_block.span,
            Block::Await(await_block) => await_block.span,
            Block::Key(key) => key.span,
            Block::Snippet(snippet) => snippet.span,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: EachBlockMetadata<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct EachBlockMetadata<'a> {
    pub contains_group_binding: bool,
    pub array_name: Option<IdentifierName<'a>>,
    pub index: IdentifierName<'a>,
    pub item: IdentifierName<'a>,
    pub declarations: FxHashMap<Atom<'a>, Binding<'a>>,
    pub references: Vec<'a, Binding<'a>>,
    pub is_controlled: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct KeyBlock<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub expression: Expression<'a>,
    pub fragment: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct SnippetBlock<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub expression: IdentifierName<'a>,
    pub parameters: Vec<'a, BindingPattern<'a>>,
    pub body: Fragment<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Comment<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub data: Atom<'a>,
    pub ignores: Vec<'a, Atom<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum SvelteNode<'a> {
    Node(Node<'a>),
    TemplateNode(TemplateNode<'a>),
    Fragment(Fragment<'a>),
    CssNode(CssNode<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Node<'a> {
    AssignmentProperty(ObjectProperty<'a>),
    CatchClause(CatchClause<'a>),
    Class(Class<'a>),
    ClassBody(ClassBody<'a>),
    Expression(Expression<'a>),
    Function(Function<'a>),
    Identifier(IdentifierName<'a>),
    Literal(Literal<'a>),
    MethodDefinition(MethodDefinition<'a>),
    ModuleDeclaration(ModuleDeclaration<'a>),
    ModuleSpecifier(ModuleSpecifier<'a>),
    Pattern(BindingPattern<'a>),
    PrivateIdentifier(PrivateIdentifier<'a>),
    Program(Program<'a>),
    // TODO: add `Property` variant
    PropertyDefinition(PropertyDefinition<'a>),
    SpreadElement(SpreadElement<'a>),
    Statement(Statement<'a>),
    Super(Super),
    SwitchCase(SwitchCase<'a>),
    TemplateElement(TemplateElement<'a>),
    VariableDeclrator(VariableDeclarator<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Literal<'a> {
    Null(NullLiteral),
    Number(NumericLiteral<'a>),
    Boolean(BooleanLiteral),
    String(StringLiteral<'a>),
    RegExp(RegExpLiteral<'a>),
    BigInt(BigIntLiteral<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum ModuleSpecifier<'a> {
    Import(ImportSpecifier<'a>),
    ImportDefault(ImportDefaultSpecifier<'a>),
    ImportNamespace(ImportNamespaceSpecifier<'a>),
    Export(ExportSpecifier<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum TemplateNode<'a> {
    Root(Root<'a>),
    Text(Text<'a>),
    Tag(Tag<'a>),
    ElementLike(ElementLike<'a>),
    Attribute(Attribute<'a>),
    SpreadAttribute(SpreadAttribute<'a>),
    Directive(Directive<'a>),
    Comment(Comment<'a>),
    Block(Block<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Root<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub options: Option<SvelteOptions<'a>>,
    pub fragment: Fragment<'a>,
    pub css: Option<StyleSheet<'a>>,
    pub instance: Option<Script<'a>>,
    pub module: Option<Script<'a>>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: RootMetadata,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
pub struct SvelteOptions<'a> {
    pub span: Span,
    pub runes: Option<bool>,
    pub immutable: Option<bool>,
    pub accessors: Option<bool>,
    pub preserve_whitespace: Option<bool>,
    pub namespace: Option<Namespace>,
    pub custom_element: Option<CustomElement<'a>>,
    pub attributes: Vec<'a, Attribute<'a>>,
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Namespace {
    #[default]
    Html,
    Svg,
    Foreign,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct CustomElement<'a> {
    pub tag: Atom<'a>, // true
    pub shadow: Option<CustomElementShadow>,
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct CustomElementProp<'a> {
    pub attribute: Option<Atom<'a>>,
    pub reflect: Option<bool>,
    #[cfg_attr(feature = "serialize", serde(rename = "type"))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum CustomElementExtend<'a> {
    ArrowFunction(ArrowFunctionExpression<'a>),
    Identifier(IdentifierName<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Script<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub context: ScriptContext,
    pub content: Program<'a>,
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct Attribute<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub value: AttributeValue<'a>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum AttributeValue<'a> {
    Bool(bool), // true
    Sequence(Vec<'a, AttributeSequenceValue<'a>>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum AttributeSequenceValue<'a> {
    Text(Text<'a>),
    ExpressionTag(ExpressionTag<'a>),
}

impl<'a> GetSpan for AttributeSequenceValue<'a> {
    fn span(&self) -> Span {
        match self {
            AttributeSequenceValue::Text(text) => text.span,
            AttributeSequenceValue::ExpressionTag(tag) => tag.span,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Directive<'a> {
    Animate(AnimateDirective<'a>),
    Bind(BindDirective<'a>),
    Class(ClassDirective<'a>),
    Let(LetDirective<'a>),
    On(OnDirective<'a>),
    Style(StyleDirective<'a>),
    Transition(TransitionDirective<'a>),
    Use(UseDirective<'a>),
}

impl<'a> GetSpan for Directive<'a> {
    fn span(&self) -> Span {
        match self {
            Directive::Animate(animate) => animate.span,
            Directive::Bind(bind) => bind.span,
            Directive::Class(class) => class.span,
            Directive::Let(let_directive) => let_directive.span,
            Directive::On(on) => on.span,
            Directive::Style(style) => style.span,
            Directive::Transition(transition) => transition.span,
            Directive::Use(use_directive) => use_directive.span,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct AnimateDirective<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub expression: Option<Expression<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum BindDirectiveExpression<'a> {
    Identifier(IdentifierName<'a>),
    MemberExpression(MemberExpression<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct BindDirectiveMetadata<'a> {
    pub binding_group_name: IdentifierName<'a>,
    pub parent_each_blocks: Vec<'a, &'a EachBlock<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct LetDirective<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub expression: Option<LetDirectiveExpression<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum LetDirectiveExpression<'a> {
    Identifier(IdentifierName<'a>),
    ArrayExpression(ArrayExpression<'a>),
    ObjectExpression(ObjectExpression<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum TransitionDirectiveModifier {
    Local,
    Global,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub struct UseDirective<'a> {
    #[cfg_attr(feature = "serialize", serde(flatten))]
    pub span: Span,
    pub name: Atom<'a>,
    pub expression: Option<Expression<'a>>,
}
