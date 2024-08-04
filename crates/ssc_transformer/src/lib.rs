//! Transformer

use oxc_allocator::{Allocator, Vec};
use oxc_ast::ast::Program;
use oxc_span::{SourceType, SPAN};
use ssc_analyzer::Analysis;
#[allow(clippy::wildcard_imports)]
use ssc_ast::ast::*;
use ssc_css_transformer::Transformer as CssTransformer;
use std::cell::Cell;

pub struct Transformer<'a> {
    allocator: &'a Allocator,
    analysis: Analysis<'a>,
}

impl<'a> Transformer<'a> {
    pub fn new(allocator: &'a Allocator, analysis: Analysis<'a>) -> Self {
        Self { allocator, analysis }
    }

    pub fn build(self, root: &mut Root<'a>) -> Program<'a> {
        if let (Some(style), Some(analysis)) = (&mut root.css, self.analysis.css) {
            CssTransformer::new(self.allocator, analysis.hash.as_str())
                .build(&mut style.stylesheet);
        }

        Program {
            span: SPAN,
            source_type: SourceType::default().with_module(true).with_typescript(root.ts),
            hashbang: None,
            directives: Vec::new_in(self.allocator),
            body: Vec::new_in(self.allocator),
            scope_id: Cell::new(None),
        }
    }
}
