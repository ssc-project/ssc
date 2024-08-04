use oxc_allocator::Allocator;
use oxc_diagnostics::{Error, OxcDiagnostic};
#[allow(clippy::wildcard_imports)]
use ssc_ast::{ast::*, Visit};
use ssc_css_analyzer::{Analysis as CssAnalysis, Analyzer as CssAnalyzer};
use std::mem;

#[derive(Debug)]
pub struct Analysis<'a> {
    pub css: Option<CssAnalysis<'a>>,
}

pub struct AnalyzerReturn<'a> {
    pub errors: Vec<Error>,
    pub analysis: Analysis<'a>,
}

pub struct Analyzer<'a> {
    allocator: &'a Allocator,
    errors: Vec<OxcDiagnostic>,
}

impl<'a> Analyzer<'a> {
    pub fn new(allocator: &'a Allocator) -> Self {
        Self { allocator, errors: Vec::new() }
    }

    fn take_errors(&mut self) -> Vec<Error> {
        let errors = mem::take(&mut self.errors);
        errors.into_iter().map(Error::from).collect()
    }

    fn error(&mut self, error: OxcDiagnostic) {
        self.errors.push(error);
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn build(mut self, root: &Root<'a>) -> AnalyzerReturn<'a> {
        self.visit_root(root);
        let css = if let Some(style) = &root.css {
            let ret = CssAnalyzer::new(self.allocator).build(&style.stylesheet);
            for error in ret.errors {
                self.error(error.downcast().unwrap());
            }
            Some(ret.analysis)
        } else {
            None
        };
        let errors = self.take_errors();
        AnalyzerReturn { errors, analysis: Analysis { css } }
    }
}

impl<'a> Visit<'a> for Analyzer<'a> {}
