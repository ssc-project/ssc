// Silence erroneous warnings from Rust Analyser for `#[derive(Tsify)]`
#![allow(non_snake_case)]

mod options;

use std::cell::RefCell;

use serde::Serialize;
use ssc::{
    allocator::Allocator,
    ast::Trivias,
    codegen::{Codegen, CodegenOptions},
    diagnostics::Error,
    parser::Parser,
};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::options::SscCodegenOptions;

#[wasm_bindgen(getter_with_clone)]
#[derive(Default, Tsify)]
pub struct Ssc {
    source_text: String,

    #[wasm_bindgen(readonly, skip_typescript)]
    #[tsify(type = "Root")]
    pub ast: JsValue,

    #[wasm_bindgen(readonly, skip_typescript)]
    #[tsify(type = "FragmentNode[]")]
    pub ir: JsValue,

    #[wasm_bindgen(readonly, skip_typescript, js_name = "codegenText")]
    #[serde(rename = "codegenText")]
    pub codegen_text: String,

    comments: Vec<Comment>,

    diagnostics: RefCell<Vec<Error>>,

    #[serde(skip)]
    serializer: serde_wasm_bindgen::Serializer,
}

#[derive(Clone, Tsify, Serialize)]
#[tsify(into_wasm_abi)]
pub struct Comment {
    pub value: String,
    pub start: u32,
    pub end: u32,
}

#[derive(Default, Clone, Serialize)]
pub struct Diagnostic {
    pub start: usize,
    pub end: usize,
    pub severity: String,
    pub message: String,
}

#[wasm_bindgen]
impl Ssc {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { serializer: serde_wasm_bindgen::Serializer::json_compatible(), ..Self::default() }
    }

    #[wasm_bindgen(getter = sourceText)]
    pub fn source_text(&self) -> String {
        self.source_text.clone()
    }

    #[wasm_bindgen(setter = sourceText)]
    pub fn set_source_text(&mut self, source_text: String) {
        self.diagnostics = RefCell::default();
        self.source_text = source_text;
    }

    /// Returns Array of String
    /// # Errors
    /// # Panics
    #[wasm_bindgen(js_name = getDiagnostics)]
    pub fn get_diagnostics(&self) -> Result<Vec<JsValue>, serde_wasm_bindgen::Error> {
        Ok(self
            .diagnostics
            .borrow()
            .iter()
            .flat_map(|error| {
                let Some(labels) = error.labels() else { return vec![] };
                labels
                    .map(|label| {
                        Diagnostic {
                            start: label.offset(),
                            end: label.offset() + label.len(),
                            severity: format!("{:?}", error.severity().unwrap_or_default()),
                            message: format!("{error}"),
                        }
                        .serialize(&self.serializer)
                        .unwrap()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>())
    }

    /// Returns comments
    /// # Errors
    #[wasm_bindgen(js_name = getComments)]
    pub fn get_comments(&self) -> Result<Vec<JsValue>, serde_wasm_bindgen::Error> {
        self.comments.iter().map(|c| c.serialize(&self.serializer)).collect()
    }

    /// # Errors
    /// Serde serialization error
    #[wasm_bindgen]
    pub fn run(
        &mut self,
        codegen_options: &SscCodegenOptions,
    ) -> Result<(), serde_wasm_bindgen::Error> {
        self.diagnostics = RefCell::default();

        let allocator = Allocator::default();
        let source_text = &self.source_text;

        let ret = Parser::new(&allocator, source_text).parse();

        self.comments = self.map_comments(&ret.trivias);
        self.save_diagnostics(ret.errors.into_iter().map(Error::from).collect::<Vec<_>>());

        self.ir = format!("{:#?}", ret.root.fragment.nodes).into();

        let root = allocator.alloc(ret.root);

        self.ast = root.serialize(&self.serializer)?;

        let options = CodegenOptions {
            enable_typescript: codegen_options.enable_typescript,
            ..CodegenOptions::default()
        };
        self.codegen_text = if codegen_options.whitespace {
            Codegen::<true>::new("", source_text, options).build(root).source_text
        } else {
            Codegen::<false>::new("", source_text, options).build(root).source_text
        };

        Ok(())
    }

    fn save_diagnostics(&self, diagnostics: Vec<Error>) {
        self.diagnostics.borrow_mut().extend(diagnostics);
    }

    fn map_comments(&self, trivias: &Trivias) -> Vec<Comment> {
        trivias
            .comments()
            .map(|span| Comment {
                value: span.source_text(&self.source_text).to_string(),
                start: span.start,
                end: span.end,
            })
            .collect()
    }
}
