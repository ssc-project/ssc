#![allow(clippy::print_stdout)]

use std::{env, path::Path};

use oxc_allocator::Allocator;
use ssc_ast::{
    ast::{Block, Element, Tag},
    visit::walk,
    Visit,
};
use ssc_parser::Parser;

// Instruction:
// create a `test.svelte`,
// run `cargo run -p ssc_parser --example visitor`

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).unwrap_or_else(|| "test.svelte".to_string());
    let path = Path::new(&name);
    let source_text = std::fs::read_to_string(path)?;
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source_text).parse();

    for error in ret.errors {
        let error = error.with_source_code(source_text.clone());
        println!("{error:?}");
    }

    let root = ret.root;

    let mut ast_pass = CountASTNodes::default();
    ast_pass.visit_root(&root);
    println!("{ast_pass:?}");

    Ok(())
}

#[derive(Debug, Default)]
struct CountASTNodes {
    elements: usize,
    tags: usize,
    blocks: usize,
}

impl<'a> Visit<'a> for CountASTNodes {
    fn visit_element(&mut self, element: &Element<'a>) {
        self.elements += 1;
        walk::walk_element(self, element);
    }

    fn visit_tag(&mut self, tag: &Tag<'a>) {
        self.tags += 1;
        walk::walk_tag(self, tag);
    }

    fn visit_block(&mut self, block: &Block<'a>) {
        self.blocks += 1;
        walk::walk_block(self, block);
    }
}
