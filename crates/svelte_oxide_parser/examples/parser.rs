use std::{env, path::Path};

use oxc_allocator::Allocator;
use svelte_oxide_parser::Parser;

// Instruction:
// create a `test.svelte`,
// run `cargo run -p svelte_oxide_parser --example parser`

fn main() -> Result<(), String> {
    let name = env::args().nth(1).unwrap_or_else(|| "test.svelte".to_string());
    let path = Path::new(&name);
    let source_text = std::fs::read_to_string(path).map_err(|_| format!("Missing '{name}'"))?;
    let allocator = Allocator::default();
    let now = std::time::Instant::now();
    let ret = Parser::new(&allocator, &source_text).parse();
    let elapsed_time = now.elapsed();
    println!("{}ms.", elapsed_time.as_millis());

    println!("AST:");
    println!("{}", serde_json::to_string_pretty(&ret.root).unwrap());

    println!("Comments:");
    let comments =
        ret.trivias.comments().map(|span| span.source_text(&source_text)).collect::<Vec<_>>();
    println!("{comments:?}");

    if ret.errors.is_empty() {
        println!("Parsed Successfully.");
    } else {
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
            println!("Parsed with Errors.");
        }
    }

    Ok(())
}
