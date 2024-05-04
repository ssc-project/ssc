use oxc_allocator::Allocator;
use oxc_diagnostics::Error;
use svelte_oxide_analyzer::analyze;
use svelte_oxide_common::options::{
    CombinedCompileOptions, CompileOptions, ValidatedCompileOptions,
};
use svelte_oxide_parser::parse;

pub fn compile<'a, T>(
    allocator: &'a Allocator,
    source_text: &'a str,
    options: CompileOptions,
) -> Result<(), Vec<Error>> {
    let validated_options = ValidatedCompileOptions::from(options);
    let root = parse(allocator, source_text)?;
    let root = allocator.alloc(root);

    let combined_options =
        CombinedCompileOptions::new(validated_options, root.options.as_ref());

    let _analysis = analyze(allocator, root, source_text, &combined_options)?;

    todo!()
}
