use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default, Clone, Copy)]
pub struct SscCodegenOptions {
    pub indentation: u8,
    #[wasm_bindgen(js_name = enableTypescript)]
    pub enable_typescript: bool,
    pub whitespace: bool,
}

#[wasm_bindgen]
impl SscCodegenOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}
