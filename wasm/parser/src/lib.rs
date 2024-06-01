// Silence erroneous warnings from Rust Analyser for `#[derive(Tsify)]`
#![allow(non_snake_case)]
#![allow(clippy::needless_pass_by_value)]

use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use ssc::{allocator::Allocator, parser::Parser};

#[derive(Default, Tsify)]
#[wasm_bindgen(getter_with_clone)]
pub struct ParseResult {
    #[wasm_bindgen(readonly, skip_typescript)]
    #[tsify(type = "Root")]
    pub root: JsValue,

    #[wasm_bindgen(readonly, skip_typescript)]
    #[tsify(type = "Diagnostic[]")]
    pub errors: Vec<JsValue>,
}

#[derive(Debug, Default, Serialize, Tsify)]
pub struct Diagnostic {
    pub start: usize,
    pub end: usize,
    pub severity: String,
    pub message: String,
}

/// # Errors
///
/// * wasm bindgen serialization failed
///
/// # Panics
///
/// * Serde JSON serialization
#[wasm_bindgen(js_name = parseSync)]
pub fn parse_sync(source_text: String) -> Result<ParseResult, serde_wasm_bindgen::Error> {
    let allocator = Allocator::default();

    let ret = Parser::new(&allocator, &source_text).parse();

    let serializer = serde_wasm_bindgen::Serializer::json_compatible();

    let root = ret.root.serialize(&serializer)?;

    let errors = if ret.errors.is_empty() {
        vec![]
    } else {
        ret.errors
            .iter()
            .flat_map(|error| {
                let Some(labels) = &error.labels else { return vec![] };
                labels
                    .iter()
                    .map(|label| {
                        Diagnostic {
                            start: label.offset(),
                            end: label.offset() + label.len(),
                            severity: "Error".to_string(),
                            message: format!("{error}"),
                        }
                        .serialize(&serializer)
                        .unwrap()
                    })
                    .collect::<Vec<JsValue>>()
            })
            .collect::<Vec<JsValue>>()
    };

    Ok(ParseResult { root, errors })
}
