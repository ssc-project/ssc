pub mod css;
pub mod template;

use oxc_allocator::Vec;
use oxc_ast::ast::{
    Class, Expression, Function, IdentifierName, ImportDeclaration,
};
use oxc_span::Atom;
#[cfg(feature = "serialize")]
use serde::Serialize;

use self::template::{EachBlock, SvelteNode};

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct Binding<'a> {
    pub node: IdentifierName<'a>,
    pub kind: BindingKind,
    pub declaration_kind: DeclarationKind,
    pub initial: Option<BindingInitial<'a>>,
    pub is_called: bool,
    pub references: BindingReferences<'a>,
    pub mutated: bool,
    pub reassigned: bool,
    // TODO: add scope
    // pub scope: Scope,
    pub legacy_dependencies: Vec<'a, Binding<'a>>,
    pub prop_alias: Option<Atom<'a>>,
    // TODO: add `expression` and mutation fields
    // pub expression: BindingExpression<'a>,
    // pub mutation: BindingMutation<'a>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub metadata: Option<BindingMetadata>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "snake_case"))]
pub enum BindingKind {
    Normal,
    Prop,
    BindableProp,
    RestProp,
    State,
    FrozenState,
    Derived,
    Each,
    Snippet,
    StoreSub,
    LegacyReactive,
    LegacyReactiveImport,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "snake_case"))]
pub enum DeclarationKind {
    Var,
    Let,
    Const,
    Function,
    Import,
    Param,
    RestParam,
    Synthetic,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum BindingInitial<'a> {
    Expression(Expression<'a>),
    FunctionDeclaration(Function<'a>),
    ClassDeclaration(Class<'a>),
    ImportDeclaration(ImportDeclaration<'a>),
    EachBlock(EachBlock<'a>),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct BindingReferences<'a> {
    pub node: IdentifierName<'a>,
    pub path: Vec<'a, SvelteNode<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct BindingMetadata {
    pub inside_rest: bool,
}
