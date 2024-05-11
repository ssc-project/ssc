//! # Svelte Oxide AST
//!
//! ## Cargo Features
//! * `"serde"` enables support for serde serialization

pub mod ast;
mod ast_builder;
mod ast_kind;
mod span;
pub mod visit;

pub use crate::{
    ast_builder::AstBuilder,
    ast_kind::{AstKind, AstType},
    visit::{Visit, VisitMut},
};

#[test]
fn lifetime_variance() {
    use crate::ast;

    fn _assert_program_variant_lifetime<'a: 'b, 'b>(
        root: ast::Root<'a>,
    ) -> ast::Root<'b> {
        root
    }
}
