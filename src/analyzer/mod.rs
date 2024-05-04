use oxc_allocator::Allocator;
use oxc_diagnostics::Error;

use crate::{ast::template::Root, options::CombinedCompileOptions};

pub struct ComponentAnalysis<'a> {
    source_text: &'a str,
}

pub struct AnalyzerReturn<'a> {
    pub analysis: ComponentAnalysis<'a>,
    pub errors: Vec<Error>,
}

pub struct Analyzer<'a> {
    allocator: &'a Allocator,
    root: &'a Root<'a>,
    source_text: &'a str,
    options: &'a CombinedCompileOptions<'a>,
    errors: Vec<Error>,
}

impl<'a> Analyzer<'a> {
    pub fn new(
        allocator: &'a Allocator,
        root: &'a Root<'a>,
        source_text: &'a str,
        options: &'a CombinedCompileOptions<'a>,
    ) -> Self {
        Self { allocator, root, source_text, options, errors: Vec::new() }
    }

    fn error<T: Into<Error>>(&mut self, error: T) {
        self.errors.push(error.into());
    }

    pub fn analyze(mut self) -> AnalyzerReturn<'a> {
        let analysis = self.analyze_component();
        AnalyzerReturn { analysis, errors: self.errors }
    }

    fn analyze_component(&mut self) -> ComponentAnalysis<'a> {
        todo!()
    }
}

pub fn analyze<'a>(
    allocator: &'a Allocator,
    root: &'a Root<'a>,
    source_text: &'a str,
    options: &'a CombinedCompileOptions<'a>,
) -> Result<ComponentAnalysis<'a>, Vec<Error>> {
    let analyzer = Analyzer::new(allocator, root, source_text, options);
    let ret = analyzer.analyze();
    if !ret.errors.is_empty() {
        return Err(ret.errors);
    }
    Ok(ret.analysis)
}
