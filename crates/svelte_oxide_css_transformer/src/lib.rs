#![allow(clippy::wildcard_imports)]

//! CSS Transformer

use oxc_span::{Atom, SPAN};
use svelte_oxide_css_ast::{ast::*, VisitMut};

pub struct Transformer<'a> {
    hash: &'a str,
}

impl<'a> Transformer<'a> {
    pub fn new(hash: &'a str) -> Self {
        Self { hash }
    }

    pub fn build(mut self, stylesheet: &mut StyleSheet<'a>) {
        self.visit_stylesheet(stylesheet);
    }
}

impl<'a> VisitMut<'a> for Transformer<'a> {
    fn visit_relative_selector(&mut self, selector: &mut RelativeSelector<'a>) {
        selector.selectors.push(SimpleSelector::ClassSelector(ClassSelector {
            span: SPAN,
            name: Atom::from(self.hash),
        }));
    }
}
