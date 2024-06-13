#![allow(clippy::wildcard_imports)]

//! CSS Transformer

use oxc_allocator::{Allocator, Vec};
use oxc_span::{Atom, SPAN};
use ssc_css_ast::{ast::*, visit::walk_mut::walk_complex_selector_mut, VisitMut};

fn clone<T>(x: &T) -> T {
    #[allow(unsafe_code)]
    // SAFETY: it's safe (maybe)
    unsafe {
        std::mem::transmute_copy(x)
    }
}

pub struct Transformer<'a> {
    allocator: &'a Allocator,
    hash: &'a str,
}

impl<'a> Transformer<'a> {
    pub fn new(allocator: &'a Allocator, hash: &'a str) -> Self {
        Self { allocator, hash }
    }

    pub fn build(mut self, stylesheet: &mut StyleSheet<'a>) {
        self.visit_stylesheet(stylesheet);
    }
}

impl<'a> VisitMut<'a> for Transformer<'a> {
    fn visit_complex_selector(&mut self, selector: &mut ComplexSelector<'a>) {
        walk_complex_selector_mut(self, selector);
        let mut replaces = vec![];
        let mut appends = vec![];
        for (i, relative_selector) in selector.children.iter_mut().enumerate() {
            for j in 0..relative_selector.selectors.len() {
                println!("j: {}, len: {}", j, relative_selector.selectors.len());
                if let SimpleSelector::PseudoClassSelector(pseudo_selector) =
                    relative_selector.selectors.get_mut(j).unwrap()
                {
                    if pseudo_selector.name.as_str() == "global" {
                        if let Some(mut children) = pseudo_selector.args.take() {
                            let relative_selectors = children.children.remove(0).children;
                            let mut transformed = transform_global_selector(
                                self.allocator,
                                relative_selectors,
                                clone(relative_selector),
                                j,
                                self.hash,
                            );
                            if !transformed.is_empty() {
                                replaces.push((i, transformed.remove(0)));
                            }
                            if !transformed.is_empty() {
                                appends.push((i + 1, transformed));
                            }
                        }
                    }
                }
            }
        }

        for (i, replace) in replaces {
            *selector.children.get_mut(i).unwrap() = replace;
        }

        for (i, appends) in appends.into_iter().rev() {
            for (j, append) in appends.into_iter().enumerate() {
                selector.children.insert(i + j, append);
            }
        }
    }

    fn visit_relative_selector(&mut self, selector: &mut RelativeSelector<'a>) {
        let has_global_selector = selector.selectors.iter().any(|selector| {
            if let SimpleSelector::PseudoClassSelector(selector) = selector {
                if selector.name.as_str() == "global" {
                    return true;
                }
            }
            false
        });
        if has_global_selector {
            return;
        }
        selector.selectors.push(SimpleSelector::ClassSelector(ClassSelector {
            span: SPAN,
            name: Atom::from(self.hash),
        }));
    }
}

// example:
//   input:  `:global(.some#random > global).selector`
//   output: `.some#random > global.selector.{hash}`
//
//   input:  `:global(p)`
//   output: `p`
fn transform_global_selector<'a>(
    allocator: &'a Allocator,
    mut inner_selectors: Vec<'a, RelativeSelector<'a>>,
    mut relative_selector: RelativeSelector<'a>,
    global_selector_index: usize,
    hash: &'a str,
) -> Vec<'a, RelativeSelector<'a>> {
    let mut selectors_after_global = Vec::from_iter_in(
        relative_selector.selectors.drain((global_selector_index + 1)..),
        allocator,
    );
    let mut selectors_before_global =
        Vec::from_iter_in(relative_selector.selectors.drain(0..global_selector_index), allocator);
    if !selectors_after_global.is_empty() {
        selectors_after_global.push(SimpleSelector::ClassSelector(ClassSelector {
            span: SPAN,
            name: Atom::from(hash),
        }));
    }
    if !selectors_before_global.is_empty() {
        selectors_before_global.push(SimpleSelector::ClassSelector(ClassSelector {
            span: SPAN,
            name: Atom::from(hash),
        }));
    }
    if inner_selectors.is_empty() {
        let mut vec = Vec::new_in(allocator);
        selectors_before_global.extend(selectors_after_global);
        vec.push(RelativeSelector {
            span: SPAN,
            selectors: selectors_before_global,
            combinator: relative_selector.combinator,
            flags: relative_selector.flags,
        });
        return vec;
    }
    if inner_selectors.len() == 1 {
        let mut vec = Vec::new_in(allocator);
        let mut inner = inner_selectors.remove(0);
        selectors_before_global.extend(inner.selectors);
        selectors_before_global.extend(selectors_after_global);
        inner.selectors = selectors_before_global;
        inner.combinator = relative_selector.combinator;
        vec.push(inner);
        return vec;
    }
    let mut vec = Vec::new_in(allocator);
    let len = inner_selectors.len();
    let mut last = inner_selectors.remove(len - 1);
    let mut first = inner_selectors.remove(0);
    first.combinator = relative_selector.combinator;
    selectors_before_global.extend(first.selectors);
    first.selectors = selectors_before_global;
    last.selectors.extend(selectors_after_global);
    vec.push(first);
    vec.extend(inner_selectors);
    vec.push(last);

    vec
}
