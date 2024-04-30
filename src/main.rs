use std::{fs::File, io::Read};

use oxc_allocator::Allocator;
use svelte_compiler::parser::Parser;

fn main() {
    let mut content = String::new();
    let mut file = File::open("./index.svelte").unwrap();
    file.read_to_string(&mut content).unwrap();
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &content).parse();

    println!("Root: {:#?}", ret.root);

    for error in ret.errors {
        eprintln!("{:?}", error.with_source_code(content.clone()));
    }
}
