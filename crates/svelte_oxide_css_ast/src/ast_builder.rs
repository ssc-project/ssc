use std::mem;

use oxc_allocator::{Allocator, Box, String, Vec};
use oxc_span::{Atom, Span};

use crate::ast::*;

pub struct AstBuilder<'a> {
    pub allocator: &'a Allocator,
}

impl<'a> AstBuilder<'a> {
    pub fn new(allocator: &'a Allocator) -> Self {
        Self { allocator }
    }

    #[inline]
    pub fn alloc<T>(&self, value: T) -> Box<'a, T> {
        Box::new_in(value, self.allocator)
    }

    #[inline]
    pub fn new_vec<T>(&self) -> Vec<'a, T> {
        Vec::new_in(self.allocator)
    }

    #[inline]
    pub fn new_vec_with_capacity<T>(&self, capacity: usize) -> Vec<'a, T> {
        Vec::with_capacity_in(capacity, self.allocator)
    }

    #[inline]
    pub fn new_vec_single<T>(&self, value: T) -> Vec<'a, T> {
        let mut vec = self.new_vec_with_capacity(1);
        vec.push(value);
        vec
    }

    #[inline]
    pub fn new_vec_from_iter<T, I: IntoIterator<Item = T>>(&self, iter: I) -> Vec<'a, T> {
        Vec::from_iter_in(iter, self.allocator)
    }

    #[inline]
    pub fn new_str(&self, value: &str) -> &'a str {
        String::from_str_in(value, self.allocator).into_bump_str()
    }

    #[inline]
    pub fn new_atom(&self, value: &str) -> Atom<'a> {
        Atom::from(String::from_str_in(value, self.allocator).into_bump_str())
    }

    pub fn copy<T>(&self, src: &T) -> T {
        // SAFETY:
        // This should be safe as long as `src` is an reference from the
        // allocator. But honestly, I'm not really sure if this is safe.
        #[allow(unsafe_code)]
        unsafe {
            mem::transmute_copy(src)
        }
    }

    #[inline]
    pub fn stylesheet(
        &self,
        span: Span,
        children: Vec<'a, Rule<'a>>,
        styles: Atom<'a>,
    ) -> StyleSheet<'a> {
        StyleSheet { span, children, content: StyleSheetContent { span, styles } }
    }

    #[inline]
    pub fn at_rule(
        &self,
        span: Span,
        name: Atom<'a>,
        prelude: Atom<'a>,
        block: Option<Block<'a>>,
    ) -> AtRule<'a> {
        AtRule { span, name, prelude, block }
    }

    #[inline]
    pub fn style_rule(
        &self,
        span: Span,
        prelude: SelectorList<'a>,
        block: Block<'a>,
    ) -> StyleRule<'a> {
        StyleRule {
            span,
            prelude,
            block,
            metadata: RuleMetadata { has_local_selectors: false, is_global_block: false },
        }
    }

    #[inline]
    pub fn block(&self, span: Span, children: Vec<'a, BlockChild<'a>>) -> Block<'a> {
        Block { span, children }
    }

    #[inline]
    pub fn declaration(&self, span: Span, property: Atom<'a>, value: Atom<'a>) -> Declaration<'a> {
        Declaration { span, property, value }
    }

    #[inline]
    pub fn selector_list(
        &self,
        span: Span,
        children: Vec<'a, ComplexSelector<'a>>,
    ) -> SelectorList<'a> {
        SelectorList { span, children }
    }

    #[inline]
    pub fn complex_selector(
        &self,
        span: Span,
        children: Vec<'a, RelativeSelector<'a>>,
    ) -> ComplexSelector<'a> {
        ComplexSelector { span, children, metadata: ComplexSelectorMetadata { used: false } }
    }

    #[inline]
    pub fn relative_selector(
        &self,
        span: Span,
        combinator: Option<Combinator>,
        selectors: Vec<'a, SimpleSelector<'a>>,
    ) -> RelativeSelector<'a> {
        RelativeSelector {
            span,
            combinator,
            selectors,
            metadata: RelativeSelectorMetadata {
                is_global: false,
                is_host: false,
                root: false,
                scoped: false,
            },
        }
    }

    #[inline]
    pub fn nesting_selector(&self, span: Span) -> SimpleSelector<'a> {
        SimpleSelector::NestingSelector(NestingSelector { span })
    }

    #[inline]
    pub fn type_selector(&self, span: Span, name: Atom<'a>) -> SimpleSelector<'a> {
        SimpleSelector::TypeSelector(TypeSelector { span, name })
    }

    #[inline]
    pub fn id_selector(&self, span: Span, name: Atom<'a>) -> SimpleSelector<'a> {
        SimpleSelector::IdSelector(IdSelector { span, name })
    }

    #[inline]
    pub fn class_selector(&self, span: Span, name: Atom<'a>) -> SimpleSelector<'a> {
        SimpleSelector::ClassSelector(ClassSelector { span, name })
    }

    #[inline]
    pub fn pseudo_element_selector(&self, span: Span, name: Atom<'a>) -> SimpleSelector<'a> {
        SimpleSelector::PseudoElementSelector(PseudoElementSelector { span, name })
    }

    #[inline]
    pub fn pseudo_class_selector(
        &self,
        span: Span,
        name: Atom<'a>,
        args: Option<SelectorList<'a>>,
    ) -> SimpleSelector<'a> {
        SimpleSelector::PseudoClassSelector(PseudoClassSelector { span, name, args })
    }

    #[inline]
    pub fn attribute_selector(
        &self,
        span: Span,
        name: Atom<'a>,
        matcher: Option<AttributeMatcher>,
        value: Option<Atom<'a>>,
        flags: Option<Atom<'a>>,
    ) -> SimpleSelector<'a> {
        SimpleSelector::AttributeSelector(AttributeSelector { span, name, matcher, value, flags })
    }

    #[inline]
    pub fn nth_selector(&self, span: Span, value: Atom<'a>) -> SimpleSelector<'a> {
        SimpleSelector::NthSelector(NthSelector { span, value })
    }

    #[inline]
    pub fn percentage_selector(&self, span: Span, value: Atom<'a>) -> SimpleSelector<'a> {
        SimpleSelector::PercentageSelector(PercentageSelector { span, value })
    }

    #[inline]
    pub fn combinator(&self, span: Span, kind: CombinatorKind) -> Combinator {
        Combinator { span, kind }
    }
}
