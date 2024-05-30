//! Svelte Oxide Codegen
//!
//! Supports
//!
//! * whitespace removal
//! * sourcemaps
//!
//! Code adapted from
//! * [esbuild](https://github.com/evanw/esbuild/blob/main/internal/js_printer/js_printer.go)

mod gen;
mod sourcemap_builder;

use sourcemap_builder::SourcemapBuilder;
#[allow(clippy::wildcard_imports)]
use svelte_oxide_ast::ast::*;

pub use crate::gen::Gen;

#[derive(Debug, Default, Clone)]
pub struct CodegenOptions {
    /// Pass in the filename to enable source map support.
    pub enable_source_map: bool,

    /// Enable TypeScript code generation.
    pub enable_typescript: bool,
}

pub struct CodegenReturn {
    pub source_text: String,
    pub source_map: Option<oxc_sourcemap::SourceMap>,
}

pub struct Codegen<const MINIFY: bool> {
    #[allow(unused)]
    options: CodegenOptions,

    /// Output Code
    code: Vec<u8>,

    /// Track the current indentation level
    indentation: u8,

    sourcemap_builder: Option<SourcemapBuilder>,
}

#[derive(Debug, Clone, Copy)]
pub enum Separator {
    Comma,
    Semicolon,
    None,
}

impl<const MINIFY: bool> Codegen<MINIFY> {
    pub fn new(source_name: &str, source_text: &str, options: CodegenOptions) -> Self {
        // Initialize the output code buffer to reduce memory reallocation.
        // Minification will reduce by at least half of the original size.
        let source_len = source_text.len();
        let capacity = if MINIFY { source_len / 2 } else { source_len };

        let sourcemap_builder = options.enable_source_map.then(|| {
            let mut sourcemap_builder = SourcemapBuilder::default();
            sourcemap_builder.with_name_and_source(source_name, source_text);
            sourcemap_builder
        });

        Self {
            options,
            // mangler: None,
            code: Vec::with_capacity(capacity),
            indentation: 0,
            sourcemap_builder,
        }
    }

    pub fn build(mut self, root: &Root<'_>) -> CodegenReturn {
        root.gen(&mut self);
        let source_text = self.into_source_text();
        let source_map = self.sourcemap_builder.map(SourcemapBuilder::into_sourcemap);
        CodegenReturn { source_text, source_map }
    }

    pub fn into_source_text(&mut self) -> String {
        // SAFETY: criteria of `from_utf8_unchecked` are met.
        #[allow(unsafe_code)]
        unsafe {
            String::from_utf8_unchecked(std::mem::take(&mut self.code))
        }
    }

    /// Push a single character into the buffer
    pub fn print(&mut self, ch: u8) {
        self.code.push(ch);
    }

    /// Push a string into the buffer
    pub fn print_str(&mut self, s: &[u8]) {
        self.code.extend_from_slice(s);
    }

    pub fn print_str_with_indention(&mut self, s: &[u8]) {
        let lines = s.split(|&ch| ch == b'\n');

        for line in lines {
            self.print_indent();
            self.print_str(line);
            self.print_soft_newline();
        }
    }

    fn print_soft_space(&mut self) {
        if !MINIFY {
            self.print(b' ');
        }
    }

    pub fn print_hard_space(&mut self) {
        self.print(b' ');
    }

    fn print_soft_newline(&mut self) {
        if !MINIFY {
            self.print(b'\n');
        }
    }

    fn indent(&mut self) {
        if !MINIFY {
            self.indentation += 1;
        }
    }

    fn dedent(&mut self) {
        if !MINIFY {
            self.indentation -= 1;
        }
    }

    fn print_indent(&mut self) {
        if !MINIFY {
            for _ in 0..self.indentation {
                self.print(b'\t');
            }
        }
    }

    fn add_source_mapping(&mut self, position: u32) {
        if let Some(sourcemap_builder) = self.sourcemap_builder.as_mut() {
            sourcemap_builder.add_source_mapping(&self.code, position, None);
        }
    }
}
