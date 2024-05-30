use std::{env, path::Path};

use oxc_allocator::Allocator;
use svelte_oxide_codegen::{Codegen, CodegenOptions};
use svelte_oxide_parser::Parser;

// Instruction:
// 1. create a `test.svelte`
// 2. run `cargo run -p svelte_oxide_codegen --example codegen`

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).unwrap_or_else(|| "test.svelte".to_string());
    let path = Path::new(&name);
    let source_text = std::fs::read_to_string(path)?;
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source_text).parse();

    if !ret.errors.is_empty() {
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
        return Ok(());
    }

    println!("Original:");
    println!("{source_text}");

    let options = CodegenOptions { enable_source_map: false, enable_typescript: true };
    let printed =
        Codegen::<false>::new("", &source_text, options.clone()).build(&ret.root).source_text;
    println!("Printed:");
    println!("{printed}");

    let ret = Parser::new(&allocator, &printed).parse();
    let minified = Codegen::<true>::new("", &source_text, options).build(&ret.root).source_text;
    println!("Minified:");
    println!("{minified}");

    Ok(())
}
