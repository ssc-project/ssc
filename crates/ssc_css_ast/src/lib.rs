//! # SSC CSS AST
//!
//! ## Cargo Features
//! * `"serialize"` enables support for serde serialization

pub mod ast;
mod ast_builder;
mod ast_kind;
mod trivia;
pub mod visit;

pub use crate::{
    ast_builder::AstBuilder,
    ast_kind::{AstKind, AstType},
    trivia::{Comment, Trivias, TriviasMap},
    visit::{Visit, VisitMut},
};

#[test]
fn lifetime_variance() {
    use crate::ast;

    fn _assert_stylesheet_variant_lifetime<'a: 'b, 'b>(
        stylesheet: ast::StyleSheet<'a>,
    ) -> ast::StyleSheet<'b> {
        stylesheet
    }
}
