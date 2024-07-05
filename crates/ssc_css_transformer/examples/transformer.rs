#![allow(clippy::print_stdout)]

use std::{env, path::Path};

use oxc_allocator::Allocator;
use ssc_css_codegen::{Codegen, CodegenOptions};
use ssc_css_parser::Parser;
use ssc_css_transformer::Transformer;

// Instruction:
// create a `test.css`,
// run `cargo run -p ssc_css_transformer --example transformer`

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

    println!("Original:\n");
    println!("{source_text}\n");

    let mut stylesheet = ret.stylesheet;
    let hash = "svelte-nb87h768";
    Transformer::new(&allocator, hash).build(&mut stylesheet);

    let printed = Codegen::<false>::new("", &source_text, CodegenOptions::default())
        .build(&stylesheet)
        .source_text;
    println!("Transformed:\n");
    println!("{printed}");
}
