#![allow(clippy::print_stdout)]
use std::{env, path::Path};

use oxc_allocator::Allocator;
use ssc_css_analyzer::Analyzer;
use ssc_css_parser::Parser;

// Instruction:
// create a `test.css`,
// run `cargo run -p ssc_css_analyzer --example analyzer`

fn main() {
    let name = env::args().nth(1).unwrap_or_else(|| "test.css".to_string());
    let path = Path::new(&name);
    let source_text = std::fs::read_to_string(path).expect("{name} not found");
    let allocator = Allocator::default();

    let ret = Parser::new(&allocator, &source_text).parse();

    if !ret.errors.is_empty() {
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
        return;
    }

    let stylesheet = allocator.alloc(ret.stylesheet);

    let ret = Analyzer::new().build(stylesheet);
    if !ret.errors.is_empty() {
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
    }

    println!("AST:");
    println!("{stylesheet:#?}");

    println!("Analysis:");
    println!("{:#?}", ret.analysis);
}
