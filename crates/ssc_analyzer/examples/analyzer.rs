#![allow(clippy::print_stdout)]
use std::{env, path::Path};

use oxc_allocator::Allocator;
use ssc_analyzer::Analyzer;
use ssc_parser::Parser;

// Instruction:
// create a `test.svelte`,
// run `cargo run -p ssc_analyzer --example analyzer`

fn main() {
    let name = env::args().nth(1).unwrap_or_else(|| "test.svelte".to_string());
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

    let root = &ret.root;

    let ret = Analyzer::new(&allocator).build(root);
    if !ret.errors.is_empty() {
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
    }

    println!("AST:");
    println!("{root:#?}");

    println!("Analysis:");
    println!("{:#?}", ret.analysis);
}
