mod diagnostics;
mod hash;
pub mod node;

use std::mem;

use hash::hash;
use node::{AstNode, AstNodes};
use oxc_diagnostics::{Error, OxcDiagnostic};
use oxc_span::{Atom, GetSpan};
#[allow(clippy::wildcard_imports)]
use ssc_css_ast::ast::*;
use ssc_css_ast::{
    visit::walk::{walk_at_rule, walk_complex_selector, walk_nesting_selector, walk_style_rule},
    AstKind, Visit,
};

pub struct Analyzer<'a> {
    errors: Vec<OxcDiagnostic>,
    keyframes: Vec<Atom<'a>>,
    current_node_id: AstNodeId,
    nodes: AstNodes<'a>,
    block_stack: Vec<AstNodeId>,
    style_rule_stack: Vec<AstNodeId>,
}

#[derive(Debug)]
pub struct Analysis<'a> {
    pub keyframes: Vec<Atom<'a>>,
    pub hash: String,
    pub nodes: AstNodes<'a>,
}

pub struct AnalyzerReturn<'a> {
    pub errors: Vec<Error>,
    pub analysis: Analysis<'a>,
}

impl<'a> Default for Analyzer<'a> {
    fn default() -> Self {
        Self {
            errors: vec![],
            keyframes: vec![],
            current_node_id: AstNodeId::new(0),
            nodes: AstNodes::default(),
            block_stack: vec![],
            style_rule_stack: vec![],
        }
    }
}

impl<'a> Analyzer<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    fn take_errors(&mut self) -> Vec<Error> {
        let errors = mem::take(&mut self.errors);
        errors.into_iter().map(Error::from).collect()
    }

    fn error(&mut self, error: OxcDiagnostic) {
        self.errors.push(error);
    }

    fn create_ast_node(&mut self, kind: AstKind<'a>) {
        let ast_node = AstNode::new(kind);
        self.current_node_id = if matches!(kind, AstKind::StyleSheet(_)) {
            let id = self.nodes.add_node(ast_node, None);
            #[allow(unsafe_code)]
            // SAFETY: `ast_node` is a `StyleSheet` and hence the root of the tree.
            unsafe {
                self.nodes.set_root(&ast_node);
            }
            id
        } else {
            self.nodes.add_node(ast_node, Some(self.current_node_id))
        };
    }

    fn pop_ast_node(&mut self) {
        if let Some(parent_id) = self.nodes.parent_id(self.current_node_id) {
            self.current_node_id = parent_id;
        }
    }

    pub fn build(mut self, stylesheet: &mut StyleSheet<'a>) -> AnalyzerReturn<'a> {
        self.visit_stylesheet(stylesheet);
        let errors = self.take_errors();
        AnalyzerReturn {
            analysis: Analysis {
                keyframes: self.keyframes,
                nodes: self.nodes,
                hash: format!("svelte-{}", hash(stylesheet.source.as_str())),
            },
            errors,
        }
    }
}

impl<'a> Visit<'a> for Analyzer<'a> {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        self.create_ast_node(kind);
        self.enter_kind(kind);
    }

    fn leave_node(&mut self, kind: AstKind<'a>) {
        self.leave_kind(kind);
        self.pop_ast_node();
    }

    fn visit_at_rule(&mut self, rule: &AtRule<'a>) {
        if is_keyframe_node(&rule.name) && !rule.prelude.starts_with("-global-") {
            self.keyframes.push(rule.prelude.clone());
        }
        walk_at_rule(self, rule);
    }

    fn visit_nesting_selector(&mut self, selector: &NestingSelector) {
        if self.block_stack.is_empty() {
            self.error(diagnostics::invalid_nesting_selector_placement(selector.span));
        }
        walk_nesting_selector(self, selector);
    }

    fn visit_complex_selector(&mut self, selector: &ComplexSelector<'a>) {
        walk_complex_selector(self, selector);

        if let Some(id) = self.style_rule_stack.last() {
            selector.rule.set(Some(*id));
        }

        selector.used.set(selector.children.iter().all(|relative_selector| {
            let flags = relative_selector.flags.get();
            flags.has_global() || flags.has_global_like()
        }));

        // ensure `:global(...)` is not used in the middle of a selector
        'ensure_valid_global_selector: {
            let Some(a) = selector.children.iter().position(|child| !is_global(child)) else {
                break 'ensure_valid_global_selector;
            };
            let Some(b) = selector.children.iter().rposition(|child| !is_global(child)) else {
                break 'ensure_valid_global_selector;
            };
            if a != b {
                for i in a..b {
                    // `i` must be between `a` and `b` which both are valid index that makes `i` a valid index too
                    if is_global(&selector.children[i]) {
                        self.error(diagnostics::invalid_global_placement(
                            selector.children[i].selectors[0].span(),
                        ));
                        return;
                    }
                }
            }

            // ensure `:global(...)` do not lead to invalid css after `:global()` is removed
            for relative_selector in &selector.children {
                for (i, simple_selector) in relative_selector.selectors.iter().enumerate() {
                    if let SimpleSelector::PseudoClassSelector(pseudo_class_selector) =
                        simple_selector
                    {
                        if pseudo_class_selector.name.as_str() == "global" {
                            let child = pseudo_class_selector
                                .args
                                .as_ref()
                                .and_then(|args| args.children.first())
                                .and_then(|first| first.children.first())
                                .and_then(|first| first.selectors.first());

                            // ensure `:global(element)` to be at the first position in a compound selector
                            if matches!(child, Some(SimpleSelector::TypeSelector(_))) && i != 0 {
                                self.error(diagnostics::invalid_global_selector_list(
                                    pseudo_class_selector.span,
                                ));
                            }

                            // ensure `:global(.class)` is not followed by a type selector, eg: `:global(.class)element`
                            if let Some(SimpleSelector::TypeSelector(selector)) =
                                relative_selector.selectors.get(i + 1)
                            {
                                self.error(diagnostics::invalid_type_selector_placement(
                                    selector.span,
                                ));
                            }

                            // ensure `:global(...)` contains a single selector
                            // (standalone :global() with multiple selectors is OK)
                            if let Some(args) = &pseudo_class_selector.args {
                                if args.children.len() > 1
                                    && (selector.children.len() > 1
                                        || relative_selector.selectors.len() > 1)
                                {
                                    self.error(diagnostics::invalid_global_selector(
                                        pseudo_class_selector.span,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn visit_relative_selector(&mut self, selector: &RelativeSelector<'a>) {
        let mut flags = selector.flags.get();
        if selector.selectors.len() >= 1 {
            if let SimpleSelector::PseudoClassSelector(pseudo_class_selector) =
                &selector.selectors[0]
            {
                if pseudo_class_selector.name.as_str() == "global"
                    && selector.selectors.iter().all(|selector| {
                        matches!(
                            selector,
                            SimpleSelector::PseudoClassSelector(_)
                                | SimpleSelector::PseudoElementSelector(_)
                        )
                    })
                {
                    flags |= RelativeSelectorFlags::Global;
                }
            }
        }

        if is_global_like(selector) {
            flags |= RelativeSelectorFlags::GlobalLike;
        }

        selector.flags.set(flags);
    }

    fn visit_style_rule(&mut self, rule: &StyleRule<'a>) {
        if let Some(id) = self.style_rule_stack.last() {
            rule.parent_rule.set(Some(*id));
        }

        let is_global_block = rule.prelude.children.iter().any(|selector| {
            let last = &selector.children[selector.children.len() - 1];
            let selector = &last.selectors[last.selectors.len() - 1];

            if let SimpleSelector::PseudoClassSelector(selector) = selector {
                selector.name.as_str() == "global" && selector.args.is_none()
            } else {
                false
            }
        });

        if is_global_block {
            rule.flags.set(rule.flags.get() | RuleFlags::GlobalBlock);

            if rule.prelude.children.len() > 1 {
                self.error(diagnostics::invalid_global_block_list(rule.prelude.span));
            }

            let complex_selector = &rule.prelude.children[0];
            let relative_selector = &complex_selector.children[complex_selector.children.len() - 1];

            if relative_selector.selectors.len() > 1 {
                self.error(diagnostics::invalid_global_block_modifier(
                    relative_selector.selectors[relative_selector.selectors.len() - 1].span(),
                ));
            }

            if let Some(combinator) = &relative_selector.combinator {
                if combinator.kind != CombinatorKind::Descendant {
                    self.error(diagnostics::invalid_global_block_combinator(
                        relative_selector.span,
                        combinator.kind,
                    ));
                }
            }

            for child in &rule.block.children {
                if let BlockChild::Declaration(declaration) = child {
                    self.error(diagnostics::invalid_global_block_declaration(declaration.span));
                }
            }
        }

        walk_style_rule(self, rule);
    }
}

impl<'a> Analyzer<'a> {
    fn enter_kind(&mut self, kind: AstKind<'a>) {
        #[allow(clippy::single_match)]
        match kind {
            AstKind::Block(_) => {
                self.block_stack.push(self.current_node_id);
            }
            AstKind::StyleRule(_) => {
                self.style_rule_stack.push(self.current_node_id);
            }
            _ => {}
        }
    }

    fn leave_kind(&mut self, kind: AstKind<'a>) {
        #[allow(clippy::single_match)]
        match kind {
            AstKind::Block(_) => {
                self.block_stack.pop();
            }
            AstKind::StyleRule(_) => {
                self.style_rule_stack.pop();
            }
            _ => {}
        }
    }
}

fn is_global_like(selector: &RelativeSelector<'_>) -> bool {
    if selector.selectors.len() == 1 {
        let first = &selector.selectors[0];

        match first {
            SimpleSelector::PseudoClassSelector(selector) => selector.name.as_str() == "host",
            SimpleSelector::PseudoElementSelector(selector) => {
                matches!(
                    selector.name.as_str(),
                    "view-transition"
                        | "view-transition-group"
                        | "view-transition-old"
                        | "view-transition-new"
                        | "view-transition-image-pair"
                )
            }
            _ => false,
        }
    } else {
        selector.selectors.iter().position(|selector| {
            if let SimpleSelector::PseudoClassSelector(selector) = selector {
                selector.name.as_str() == "root"
            } else {
                false
            }
        }) != Some(0)
    }
}

fn is_global(selector: &RelativeSelector<'_>) -> bool {
    let Some(SimpleSelector::PseudoClassSelector(first)) = selector.selectors.first() else {
        return false;
    };
    first.name.as_str() == "global"
        && selector.selectors.iter().all(|selector| {
            matches!(
                selector,
                SimpleSelector::PseudoClassSelector(_) | SimpleSelector::PseudoElementSelector(_)
            )
        })
}

fn remove_css_prefix(name: &str) -> &str {
    if let Some(stripped) = name.strip_prefix("-webkit-") {
        return stripped;
    }
    if let Some(stripped) = name.strip_prefix("-moz-") {
        return stripped;
    }
    if let Some(stripped) = name.strip_prefix("-o-") {
        return stripped;
    }
    if let Some(stripped) = name.strip_prefix("-ms-") {
        return stripped;
    }
    name
}

fn is_keyframe_node(name: &str) -> bool {
    remove_css_prefix(name) == "keyframes"
}
