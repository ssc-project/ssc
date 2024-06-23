#![allow(clippy::wildcard_imports)]

//! # SSC AST
//!
//! ## Cargo Features
//! * `"serialize"` enables support for serde serialization

pub mod ast;
mod ast_builder;
mod ast_kind;
mod span;
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

    fn _assert_root_variant_lifetime<'a: 'b, 'b>(root: ast::Root<'a>) -> ast::Root<'b> {
        root
    }
}
