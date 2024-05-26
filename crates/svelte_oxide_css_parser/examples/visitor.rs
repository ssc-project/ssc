use std::{env, path::Path};

use oxc_allocator::Allocator;
use svelte_oxide_css_ast::{
    ast::{AtRule, Declaration, StyleRule},
    visit::walk,
    Visit,
};
use svelte_oxide_css_parser::Parser;

// Instruction:
// create a `test.css`,
// run `cargo run -p svelte_oxide_css_parser --example visitor`

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).unwrap_or_else(|| "test.css".to_string());
    let path = Path::new(&name);
    let source_text = std::fs::read_to_string(path)?;
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source_text).parse();

    for error in ret.errors {
        let error = error.with_source_code(source_text.clone());
        println!("{error:?}");
    }

    let program = ret.stylesheet;

    let mut ast_pass = CountASTNodes::default();
    ast_pass.visit_stylesheet(&program);
    println!("{ast_pass:?}");

    Ok(())
}

#[derive(Debug, Default)]
struct CountASTNodes {
    style_rules: usize,
    at_rules: usize,
    declarations: usize,
}

impl<'a> Visit<'a> for CountASTNodes {
    fn visit_style_rule(&mut self, rule: &StyleRule<'a>) {
        self.style_rules += 1;
        walk::walk_style_rule(self, rule);
    }

    fn visit_at_rule(&mut self, rule: &AtRule<'a>) {
        self.at_rules += 1;
        walk::walk_at_rule(self, rule);
    }

    fn visit_declaration(&mut self, decl: &Declaration<'a>) {
        self.declarations += 1;
        walk::walk_declaration(self, decl);
    }
}
