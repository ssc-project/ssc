use oxc_sourcemap::SourceMap;

use crate::{
    ast::template::{CustomElement, Namespace, SvelteOptions},
    parser::utils::copy_ast_node,
};

#[derive(Debug, Default)]
pub struct CompileOptions {
    pub name: Option<String>,
    pub custom_element: Option<bool>,
    pub accessors: Option<bool>,
    pub namespace: Option<Namespace>,
    pub immutable: Option<bool>,
    pub css: Option<CssMode>,
    // TODO: add this
    // pub css_hasher: Option<CssHasher>,
    pub preserve_comments: Option<bool>,
    pub preserve_whitespace: Option<bool>,
    pub runes: Option<bool>,
    pub disclose_version: Option<bool>,
    pub legacy: Option<LegacyCompileOptions>,
    pub source_map: Option<SourceMap>,
    pub output_filename: Option<String>,
    pub css_output_filename: Option<String>,
    pub hmr: Option<bool>,
    // TODO: look at it later
    pub modern_ast: Option<bool>,
    pub dev: Option<bool>,
    pub generate: Option<GenerateMode>,
    pub filename: Option<String>,
}

#[derive(Debug, Default)]
pub enum CssMode {
    Injected,
    #[default]
    Expternal,
}

#[derive(Debug, Default)]
pub enum GenerateMode {
    #[default]
    Client,
    Server,
    None,
}

#[derive(Debug, Default)]
pub struct LegacyCompileOptions {
    component_api: Option<bool>,
}

#[derive(Debug)]
pub(crate) struct ValidatedCompileOptions {
    pub name: Option<String>,
    pub custom_element: bool,
    pub accessors: bool,
    pub namespace: Namespace,
    pub immutable: bool,
    pub css: CssMode,
    // TODO: add this
    // pub css_hasher: Option<CssHasher>,
    pub preserve_comments: bool,
    pub preserve_whitespace: bool,
    pub runes: Option<bool>,
    pub disclose_version: bool,
    pub legacy: LegacyCompileOptions,
    pub source_map: Option<SourceMap>,
    pub output_filename: Option<String>,
    pub css_output_filename: Option<String>,
    pub hmr: bool,
    // TODO: look at it later
    pub modern_ast: Option<bool>,
    pub dev: bool,
    pub generate: GenerateMode,
    pub filename: Option<String>,
}

#[derive(Debug, Default)]
pub(crate) struct ValidatedLegacyCompileOptions {
    component_api: bool,
}

impl From<CompileOptions> for ValidatedCompileOptions {
    fn from(compile_options: CompileOptions) -> Self {
        Self {
            name: compile_options.name,
            custom_element: compile_options.custom_element.unwrap_or_default(),
            accessors: compile_options.accessors.unwrap_or_default(),
            namespace: compile_options.namespace.unwrap_or_default(),
            immutable: compile_options.immutable.unwrap_or_default(),
            css: compile_options.css.unwrap_or_default(),
            preserve_comments: compile_options
                .preserve_comments
                .unwrap_or_default(),
            preserve_whitespace: compile_options
                .preserve_whitespace
                .unwrap_or_default(),
            runes: compile_options.runes,
            disclose_version: compile_options.disclose_version.unwrap_or(true),
            legacy: compile_options.legacy.unwrap_or_default().into(),
            source_map: compile_options.source_map,
            output_filename: compile_options.output_filename,
            css_output_filename: compile_options.css_output_filename,
            hmr: compile_options.hmr.unwrap_or_default(),
            modern_ast: compile_options.modern_ast,
            dev: compile_options.dev.unwrap_or_default(),
            generate: compile_options.generate.unwrap_or_default(),
            filename: compile_options.filename,
        }
    }
}

impl From<LegacyCompileOptions> for ValidatedLegacyCompileOptions {
    fn from(legacy_compile_options: LegacyCompileOptions) -> Self {
        ValidatedLegacyCompileOptions {
            component_api: legacy_compile_options
                .component_api
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CombinedCompileOptions<'a> {
    pub name: Option<String>,
    pub custom_element: bool,
    pub accessors: bool,
    pub namespace: Namespace,
    pub immutable: bool,
    pub css: CssMode,
    // TODO: add this
    // pub css_hasher: Option<CssHasher>,
    pub preserve_comments: bool,
    pub preserve_whitespace: bool,
    pub runes: Option<bool>,
    pub disclose_version: bool,
    pub legacy: LegacyCompileOptions,
    pub source_map: Option<SourceMap>,
    pub output_filename: Option<String>,
    pub css_output_filename: Option<String>,
    pub hmr: bool,
    // TODO: look at it later
    pub modern_ast: Option<bool>,
    pub dev: bool,
    pub generate: GenerateMode,
    pub filename: Option<String>,
    pub custom_element_options: Option<CustomElement<'a>>,
}

impl<'a> CombinedCompileOptions<'a> {
    pub fn new(
        options: ValidatedCompileOptions,
        parsed_options: Option<&'a SvelteOptions<'a>>,
    ) -> Self {
        Self {
            name: options.name,
            custom_element: options.custom_element,
            accessors: parsed_options
                .and_then(|options| options.accessors)
                .unwrap_or(options.accessors),
            namespace: parsed_options
                .and_then(|options| options.namespace)
                .unwrap_or(options.namespace),
            immutable: parsed_options
                .and_then(|options| options.immutable)
                .unwrap_or(options.immutable),
            css: options.css,
            preserve_comments: options.preserve_comments,
            preserve_whitespace: parsed_options
                .and_then(|options| options.preserve_whitespace)
                .unwrap_or(options.preserve_whitespace),
            runes: parsed_options
                .and_then(|options| options.runes)
                .or(options.runes),
            disclose_version: options.disclose_version,
            legacy: options.legacy,
            source_map: options.source_map,
            output_filename: options.output_filename,
            css_output_filename: options.css_output_filename,
            hmr: options.hmr,
            modern_ast: options.modern_ast,
            dev: options.dev,
            generate: options.generate,
            filename: options.filename,
            custom_element_options: parsed_options
                .and_then(|options| copy_ast_node(&options.custom_element)),
        }
    }
}
