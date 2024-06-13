//! Visit Mut Pattern

use oxc_allocator::Vec;

use self::walk_mut::*;
use crate::{ast::*, AstType};

/// Syntax tree traversal to mutate an exclusive borrow of a syntax tree in
/// place a syntax tree in place.
pub trait VisitMut<'a>: Sized {
    fn enter_node(&mut self, _kind: AstType) {}
    fn leave_node(&mut self, _kind: AstType) {}

    fn visit_stylesheet(&mut self, stylesheet: &mut StyleSheet<'a>) {
        walk_stylesheet_mut(self, stylesheet);
    }

    /* ----------  Rule ---------- */

    fn visit_rules(&mut self, rules: &mut Vec<'a, Rule<'a>>) {
        walk_rules_mut(self, rules);
    }

    fn visit_rule(&mut self, rule: &mut Rule<'a>) {
        walk_rule_mut(self, rule);
    }

    fn visit_at_rule(&mut self, rule: &mut AtRule<'a>) {
        walk_at_rule_mut(self, rule);
    }

    fn visit_style_rule(&mut self, rule: &mut StyleRule<'a>) {
        walk_style_rule_mut(self, rule);
    }

    /* ----------  Block ---------- */

    fn visit_block(&mut self, block: &mut Block<'a>) {
        walk_block_mut(self, block);
    }

    fn visit_block_child(&mut self, child: &mut BlockChild<'a>) {
        walk_block_child_mut(self, child);
    }

    fn visit_declaration(&mut self, decl: &mut Declaration<'a>) {
        walk_declaration_mut(self, decl);
    }

    /* ----------  Selector ---------- */

    fn visit_selector_list(&mut self, selector_list: &mut SelectorList<'a>) {
        walk_selector_list_mut(self, selector_list);
    }

    fn visit_complex_selector(&mut self, selector: &mut ComplexSelector<'a>) {
        walk_complex_selector_mut(self, selector);
    }

    fn visit_relative_selector(&mut self, selector: &mut RelativeSelector<'a>) {
        walk_relative_selector_mut(self, selector);
    }

    fn visit_simple_selector(&mut self, selector: &mut SimpleSelector<'a>) {
        walk_simple_selector_mut(self, selector);
    }

    fn visit_type_selector(&mut self, selector: &mut TypeSelector<'a>) {
        walk_type_selector_mut(self, selector);
    }

    fn visit_id_selector(&mut self, selector: &mut IdSelector<'a>) {
        walk_id_selector_mut(self, selector);
    }

    fn visit_class_selector(&mut self, selector: &mut ClassSelector<'a>) {
        walk_class_selector_mut(self, selector);
    }

    fn visit_attribute_selector(&mut self, selector: &mut AttributeSelector<'a>) {
        walk_attribute_selector_mut(self, selector);
    }

    fn visit_pseudo_element_selector(&mut self, selector: &mut PseudoElementSelector<'a>) {
        walk_pseudo_element_selector_mut(self, selector);
    }

    fn visit_pseudo_class_selector(&mut self, selector: &mut PseudoClassSelector<'a>) {
        walk_pseudo_class_selector_mut(self, selector);
    }

    fn visit_percentage_selector(&mut self, selector: &mut PercentageSelector<'a>) {
        walk_percentage_selector_mut(self, selector);
    }

    fn visit_nth_selector(&mut self, selector: &mut NthSelector<'a>) {
        walk_nth_selector_mut(self, selector);
    }

    fn visit_nesting_selector(&mut self, selector: &mut NestingSelector) {
        walk_nesting_selector_mut(self, selector);
    }

    fn visit_combinator(&mut self, combinator: &mut Combinator) {
        walk_combinator_mut(self, combinator);
    }
}

pub mod walk_mut {
    use super::*;

    pub fn walk_stylesheet_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        stylesheet: &mut StyleSheet<'a>,
    ) {
        let kind = AstType::StyleSheet;
        visitor.enter_node(kind);
        visitor.visit_rules(&mut stylesheet.children);
        visitor.leave_node(kind);
    }

    /* ----------  Rule ---------- */

    pub fn walk_rules_mut<'a, V: VisitMut<'a>>(visitor: &mut V, rules: &mut Vec<'a, Rule<'a>>) {
        for rule in rules.iter_mut() {
            visitor.visit_rule(rule);
        }
    }

    pub fn walk_rule_mut<'a, V: VisitMut<'a>>(visitor: &mut V, rule: &mut Rule<'a>) {
        match rule {
            Rule::AtRule(rule) => visitor.visit_at_rule(rule),
            Rule::StyleRule(rule) => visitor.visit_style_rule(rule),
        }
    }

    pub fn walk_at_rule_mut<'a, V: VisitMut<'a>>(visitor: &mut V, rule: &mut AtRule<'a>) {
        let kind = AstType::AtRule;
        visitor.enter_node(kind);
        if let Some(block) = rule.block.as_mut() {
            visitor.visit_block(block);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_style_rule_mut<'a, V: VisitMut<'a>>(visitor: &mut V, rule: &mut StyleRule<'a>) {
        let kind = AstType::StyleRule;
        visitor.enter_node(kind);
        visitor.visit_selector_list(&mut rule.prelude);
        visitor.visit_block(&mut rule.block);
        visitor.leave_node(kind);
    }

    /* ----------  Block ---------- */

    pub fn walk_block_mut<'a, V: VisitMut<'a>>(visitor: &mut V, block: &mut Block<'a>) {
        let kind = AstType::Block;
        visitor.enter_node(kind);
        for child in block.children.iter_mut() {
            visitor.visit_block_child(child);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_block_child_mut<'a, V: VisitMut<'a>>(visitor: &mut V, child: &mut BlockChild<'a>) {
        match child {
            BlockChild::Declaration(decl) => visitor.visit_declaration(decl),
            BlockChild::StyleRule(rule) => visitor.visit_style_rule(rule),
            BlockChild::AtRule(rule) => visitor.visit_at_rule(rule),
        }
    }

    pub fn walk_declaration_mut<'a, V: VisitMut<'a>>(visitor: &mut V, _decl: &mut Declaration<'a>) {
        let kind = AstType::Declaration;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    /* ----------  Selector ---------- */

    pub fn walk_selector_list_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        selector_list: &mut SelectorList<'a>,
    ) {
        for selector in selector_list.children.iter_mut() {
            visitor.visit_complex_selector(selector);
        }
    }

    pub fn walk_complex_selector_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        selector: &mut ComplexSelector<'a>,
    ) {
        let kind = AstType::ComplexSelector;
        visitor.enter_node(kind);
        for child in selector.children.iter_mut() {
            visitor.visit_relative_selector(child);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_relative_selector_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        selector: &mut RelativeSelector<'a>,
    ) {
        let kind = AstType::RelativeSelector;
        visitor.enter_node(kind);
        if let Some(combinator) = selector.combinator.as_mut() {
            visitor.visit_combinator(combinator);
        }
        for selector in selector.selectors.iter_mut() {
            visitor.visit_simple_selector(selector);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_simple_selector_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        selector: &mut SimpleSelector<'a>,
    ) {
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

    pub fn walk_type_selector_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        _selector: &mut TypeSelector<'a>,
    ) {
        let kind = AstType::TypeSelector;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_id_selector_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        _selector: &mut IdSelector<'a>,
    ) {
        let kind = AstType::IdSelector;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_class_selector_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        _selector: &mut ClassSelector<'a>,
    ) {
        let kind = AstType::ClassSelector;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_attribute_selector_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        _selector: &mut AttributeSelector<'a>,
    ) {
        let kind = AstType::AttributeSelector;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_pseudo_element_selector_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        _selector: &mut PseudoElementSelector<'a>,
    ) {
        let kind = AstType::PseudoElementSelector;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_pseudo_class_selector_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        selector: &mut PseudoClassSelector<'a>,
    ) {
        let kind = AstType::PseudoClassSelector;
        visitor.enter_node(kind);
        if let Some(args) = selector.args.as_mut() {
            visitor.visit_selector_list(args);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_percentage_selector_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        _selector: &mut PercentageSelector<'a>,
    ) {
        let kind = AstType::PercentageSelector;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_nth_selector_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        _selector: &mut NthSelector<'a>,
    ) {
        let kind = AstType::NthSelector;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_nesting_selector_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        _selector: &mut NestingSelector,
    ) {
        let kind = AstType::NestingSelector;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_combinator_mut<'a, V: VisitMut<'a>>(visitor: &mut V, _combinator: &mut Combinator) {
        let kind = AstType::Combinator;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }
}
