use std::{env, path::Path};

use base64::{prelude::BASE64_STANDARD, Engine};
use oxc_allocator::Allocator;
use ssc_css_codegen::{Codegen, CodegenOptions, CodegenReturn};
use ssc_css_parser::Parser;

// Instruction:
// 1. create a `test.css`
// 2. run `cargo run -p ssc_css_codegen --example sourcemap`

fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).unwrap_or_else(|| "test.css".to_string());
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

    let codegen_options = CodegenOptions { enable_source_map: true };

    let CodegenReturn { source_text, source_map } =
        Codegen::<false>::new(path.to_string_lossy().as_ref(), &source_text, codegen_options)
            .build(&ret.stylesheet);

    if let Some(source_map) = source_map {
        let result = source_map.to_json_string().unwrap();
        let hash = BASE64_STANDARD.encode(format!(
            "{}\0{}{}\0{}",
            source_text.len(),
            source_text,
            result.len(),
            result
        ));
        println!("https://evanw.github.io/source-map-visualization/#{hash}");
    }

    Ok(())
}
