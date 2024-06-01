//! Visitor Pattern
//!
//! See:
//! * [visitor pattern](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html)
//! * [rustc visitor](https://github.com/rust-lang/rust/blob/master/compiler/rustc_ast/src/visit.rs)

use oxc_allocator::Vec;
use walk::*;

use crate::{ast::*, ast_kind::AstKind};

/// Syntax tree traversal
pub trait Visit<'a>: Sized {
    fn enter_node(&mut self, _kind: AstKind<'a>) {}
    fn leave_node(&mut self, _kind: AstKind<'a>) {}

    fn alloc<T>(&self, t: &T) -> &'a T {
        // SAFETY:
        // This should be safe as long as `src` is an reference from the
        // allocator. But honestly, I'm not really sure if this is safe.
        #[allow(unsafe_code)]
        unsafe {
            std::mem::transmute(t)
        }
    }

    fn visit_stylesheet(&mut self, stylesheet: &StyleSheet<'a>) {
        walk_stylesheet(self, stylesheet);
    }

    /* ----------  Rule ---------- */

    fn visit_rules(&mut self, rules: &Vec<'a, Rule<'a>>) {
        walk_rules(self, rules);
    }

    fn visit_rule(&mut self, rule: &Rule<'a>) {
        walk_rule(self, rule);
    }

    fn visit_at_rule(&mut self, rule: &AtRule<'a>) {
        walk_at_rule(self, rule);
    }

    fn visit_style_rule(&mut self, rule: &StyleRule<'a>) {
        walk_style_rule(self, rule);
    }

    /* ----------  Block ---------- */

    fn visit_block(&mut self, block: &Block<'a>) {
        walk_block(self, block);
    }

    fn visit_block_child(&mut self, child: &BlockChild<'a>) {
        walk_block_child(self, child);
    }

    fn visit_declaration(&mut self, decl: &Declaration<'a>) {
        walk_declaration(self, decl);
    }

    /* ----------  Selector ---------- */

    fn visit_selector_list(&mut self, selector_list: &SelectorList<'a>) {
        walk_selector_list(self, selector_list);
    }

    fn visit_complex_selector(&mut self, selector: &ComplexSelector<'a>) {
        walk_complex_selector(self, selector);
    }

    fn visit_relative_selector(&mut self, selector: &RelativeSelector<'a>) {
        walk_relative_selector(self, selector);
    }

    fn visit_simple_selector(&mut self, selector: &SimpleSelector<'a>) {
        walk_simple_selector(self, selector);
    }

    fn visit_type_selector(&mut self, selector: &TypeSelector<'a>) {
        walk_type_selector(self, selector);
    }

    fn visit_id_selector(&mut self, selector: &IdSelector<'a>) {
        walk_id_selector(self, selector);
    }

    fn visit_class_selector(&mut self, selector: &ClassSelector<'a>) {
        walk_class_selector(self, selector);
    }

    fn visit_attribute_selector(&mut self, selector: &AttributeSelector<'a>) {
        walk_attribute_selector(self, selector);
    }

    fn visit_pseudo_element_selector(&mut self, selector: &PseudoElementSelector<'a>) {
        walk_pseudo_element_selector(self, selector);
    }

    fn visit_pseudo_class_selector(&mut self, selector: &PseudoClassSelector<'a>) {
        walk_pseudo_class_selector(self, selector);
    }

    fn visit_percentage_selector(&mut self, selector: &PercentageSelector<'a>) {
        walk_percentage_selector(self, selector);
    }

    fn visit_nth_selector(&mut self, selector: &NthSelector<'a>) {
        walk_nth_selector(self, selector);
    }

    fn visit_nesting_selector(&mut self, selector: &NestingSelector) {
        walk_nesting_selector(self, selector);
    }

    fn visit_combinator(&mut self, combinator: &Combinator) {
        walk_combinator(self, combinator);
    }
}

pub mod walk {
    use super::*;

    pub fn walk_stylesheet<'a, V: Visit<'a>>(visitor: &mut V, stylesheet: &StyleSheet<'a>) {
        let kind = AstKind::StyleSheet(visitor.alloc(stylesheet));
        visitor.enter_node(kind);
        visitor.visit_rules(&stylesheet.children);
        visitor.leave_node(kind);
    }

    /* ----------  Rule ---------- */

    pub fn walk_rules<'a, V: Visit<'a>>(visitor: &mut V, rules: &Vec<'a, Rule<'a>>) {
        for rule in rules {
            visitor.visit_rule(rule);
        }
    }

    pub fn walk_rule<'a, V: Visit<'a>>(visitor: &mut V, rule: &Rule<'a>) {
        match rule {
            Rule::AtRule(rule) => visitor.visit_at_rule(rule),
            Rule::StyleRule(rule) => visitor.visit_style_rule(rule),
        }
    }

    pub fn walk_at_rule<'a, V: Visit<'a>>(visitor: &mut V, rule: &AtRule<'a>) {
        let kind = AstKind::AtRule(visitor.alloc(rule));
        visitor.enter_node(kind);
        if let Some(block) = rule.block.as_ref() {
            visitor.visit_block(block);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_style_rule<'a, V: Visit<'a>>(visitor: &mut V, rule: &StyleRule<'a>) {
        let kind = AstKind::StyleRule(visitor.alloc(rule));
        visitor.enter_node(kind);
        visitor.visit_selector_list(&rule.prelude);
        visitor.visit_block(&rule.block);
        visitor.leave_node(kind);
    }

    /* ----------  Block ---------- */

    pub fn walk_block<'a, V: Visit<'a>>(visitor: &mut V, block: &Block<'a>) {
        for child in &block.children {
            visitor.visit_block_child(child);
        }
    }

    pub fn walk_block_child<'a, V: Visit<'a>>(visitor: &mut V, child: &BlockChild<'a>) {
        match child {
            BlockChild::Declaration(decl) => visitor.visit_declaration(decl),
            BlockChild::StyleRule(rule) => visitor.visit_style_rule(rule),
            BlockChild::AtRule(rule) => visitor.visit_at_rule(rule),
        }
    }

    pub fn walk_declaration<'a, V: Visit<'a>>(visitor: &mut V, decl: &Declaration<'a>) {
        let kind = AstKind::Declaration(visitor.alloc(decl));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    /* ----------  Selector ---------- */

    pub fn walk_selector_list<'a, V: Visit<'a>>(visitor: &mut V, selector_list: &SelectorList<'a>) {
        for selector in &selector_list.children {
            visitor.visit_complex_selector(selector);
        }
    }

    pub fn walk_complex_selector<'a, V: Visit<'a>>(
        visitor: &mut V,
        selector: &ComplexSelector<'a>,
    ) {
        let kind = AstKind::ComplexSelector(visitor.alloc(selector));
        visitor.enter_node(kind);
        for child in &selector.children {
            visitor.visit_relative_selector(child);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_relative_selector<'a, V: Visit<'a>>(
        visitor: &mut V,
        selector: &RelativeSelector<'a>,
    ) {
        let kind = AstKind::RelativeSelector(visitor.alloc(selector));
        visitor.enter_node(kind);
        if let Some(combinator) = selector.combinator.as_ref() {
            visitor.visit_combinator(combinator);
        }
        for selector in &selector.selectors {
            visitor.visit_simple_selector(selector);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_simple_selector<'a, V: Visit<'a>>(visitor: &mut V, selector: &SimpleSelector<'a>) {
        match selector {
            SimpleSelector::TypeSelector(selector) => visitor.visit_type_selector(selector),
            SimpleSelector::IdSelector(selector) => visitor.visit_id_selector(selector),
            SimpleSelector::ClassSelector(selector) => visitor.visit_class_selector(selector),
            SimpleSelector::AttributeSelector(selector) => {
                visitor.visit_attribute_selector(selector);
            }
            SimpleSelector::PseudoElementSelector(selector) => {
                visitor.visit_pseudo_element_selector(selector);
            }
            SimpleSelector::PseudoClassSelector(selector) => {
                visitor.visit_pseudo_class_selector(selector);
            }
            SimpleSelector::PercentageSelector(selector) => {
                visitor.visit_percentage_selector(selector);
            }
            SimpleSelector::NthSelector(selector) => visitor.visit_nth_selector(selector),
            SimpleSelector::NestingSelector(selector) => visitor.visit_nesting_selector(selector),
        }
    }

    pub fn walk_type_selector<'a, V: Visit<'a>>(visitor: &mut V, selector: &TypeSelector<'a>) {
        let kind = AstKind::TypeSelector(visitor.alloc(selector));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_id_selector<'a, V: Visit<'a>>(visitor: &mut V, selector: &IdSelector<'a>) {
        let kind = AstKind::IdSelector(visitor.alloc(selector));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_class_selector<'a, V: Visit<'a>>(visitor: &mut V, selector: &ClassSelector<'a>) {
        let kind = AstKind::ClassSelector(visitor.alloc(selector));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_attribute_selector<'a, V: Visit<'a>>(
        visitor: &mut V,
        selector: &AttributeSelector<'a>,
    ) {
        let kind = AstKind::AttributeSelector(visitor.alloc(selector));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_pseudo_element_selector<'a, V: Visit<'a>>(
        visitor: &mut V,
        selector: &PseudoElementSelector<'a>,
    ) {
        let kind = AstKind::PseudoElementSelector(visitor.alloc(selector));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_pseudo_class_selector<'a, V: Visit<'a>>(
        visitor: &mut V,
        selector: &PseudoClassSelector<'a>,
    ) {
        let kind = AstKind::PseudoClassSelector(visitor.alloc(selector));
        visitor.enter_node(kind);
        if let Some(args) = selector.args.as_ref() {
            visitor.visit_selector_list(args);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_percentage_selector<'a, V: Visit<'a>>(
        visitor: &mut V,
        selector: &PercentageSelector<'a>,
    ) {
        let kind = AstKind::PercentageSelector(visitor.alloc(selector));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_nth_selector<'a, V: Visit<'a>>(visitor: &mut V, selector: &NthSelector<'a>) {
        let kind = AstKind::NthSelector(visitor.alloc(selector));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_nesting_selector<'a, V: Visit<'a>>(visitor: &mut V, selector: &NestingSelector) {
        let kind = AstKind::NestingSelector(visitor.alloc(selector));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_combinator<'a, V: Visit<'a>>(visitor: &mut V, combinator: &Combinator) {
        let kind = AstKind::Combinator(visitor.alloc(combinator));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }
}
