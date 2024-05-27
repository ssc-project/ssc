use std::mem;

use oxc_allocator::{Allocator, Box, String, Vec};
use oxc_ast::ast::{Expression, Program};
use oxc_span::{Atom, Span};
use svelte_oxide_css_ast::ast::StyleSheet;

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
    pub fn root(
        &self,
        span: Span,
        fragment: Fragment<'a>,
        css: Option<Style<'a>>,
        instance: Option<Script<'a>>,
        module: Option<Script<'a>>,
        ts: bool,
    ) -> Root<'a> {
        Root { span, options: None, fragment, css, instance, module, metadata: RootMetadata { ts } }
    }

    #[inline]
    pub fn fragment(&self, nodes: Vec<'a, FragmentNode<'a>>, transparent: bool) -> Fragment<'a> {
        Fragment { nodes, transparent }
    }

    #[inline]
    pub fn script(
        &self,
        span: Span,
        context: ScriptContext,
        content: Program<'a>,
        attributes: Vec<'a, Attribute<'a>>,
    ) -> Script<'a> {
        Script { span, context, content, attributes }
    }

    #[inline]
    pub fn style(
        &self,
        span: Span,
        stylesheet: StyleSheet<'a>,
        attributes: Vec<'a, Attribute<'a>>,
        content_span: Span,
        content: Atom<'a>,
    ) -> Style<'a> {
        Style {
            span,
            attributes,
            stylesheet,
            content: StyleContent { span: content_span, styles: content },
        }
    }

    #[inline]
    pub fn component(
        &self,
        span: Span,
        name: Atom<'a>,
        attributes: Vec<'a, ElementAttribute<'a>>,
        fragment: Fragment<'a>,
    ) -> Element<'a> {
        Element::Component(Component { span, name, attributes, fragment })
    }

    #[inline]
    pub fn title_element(
        &self,
        span: Span,
        attributes: Vec<'a, ElementAttribute<'a>>,
        fragment: Fragment<'a>,
    ) -> Element<'a> {
        Element::TitleElement(TitleElement { span, attributes, fragment })
    }

    #[inline]
    pub fn slot_element(
        &self,
        span: Span,
        attributes: Vec<'a, ElementAttribute<'a>>,
        fragment: Fragment<'a>,
    ) -> Element<'a> {
        Element::SlotElement(SlotElement { span, attributes, fragment })
    }

    #[inline]
    pub fn regular_element(
        &self,
        span: Span,
        name: Atom<'a>,
        attributes: Vec<'a, ElementAttribute<'a>>,
        fragment: Fragment<'a>,
    ) -> Element<'a> {
        Element::RegularElement(RegularElement {
            span,
            name,
            attributes,
            fragment,
            metadata: RegularElementMetadata { svg: false, has_spread: false, scoped: false },
        })
    }

    #[inline]
    pub fn svelte_body(
        &self,
        span: Span,
        attributes: Vec<'a, ElementAttribute<'a>>,
        fragment: Fragment<'a>,
    ) -> Element<'a> {
        Element::SvelteBody(SvelteBody { span, attributes, fragment })
    }

    #[inline]
    pub fn svelte_component(
        &self,
        span: Span,
        attributes: Vec<'a, ElementAttribute<'a>>,
        fragment: Fragment<'a>,
        expression: Expression<'a>,
    ) -> Element<'a> {
        Element::SvelteComponent(SvelteComponent { span, attributes, fragment, expression })
    }

    #[inline]
    pub fn svelte_document(
        &self,
        span: Span,
        attributes: Vec<'a, ElementAttribute<'a>>,
        fragment: Fragment<'a>,
    ) -> Element<'a> {
        Element::SvelteDocument(SvelteDocument { span, attributes, fragment })
    }

    #[inline]
    pub fn svelte_element(
        &self,
        span: Span,
        attributes: Vec<'a, ElementAttribute<'a>>,
        fragment: Fragment<'a>,
        expression: Expression<'a>,
    ) -> Element<'a> {
        Element::SvelteElement(SvelteElement {
            span,
            attributes,
            fragment,
            expression,
            metadata: SvelteElementMetadata { svg: false, scoped: false },
        })
    }

    #[inline]
    pub fn svelte_fragment(
        &self,
        span: Span,
        attributes: Vec<'a, ElementAttribute<'a>>,
        fragment: Fragment<'a>,
    ) -> Element<'a> {
        Element::SvelteFragment(SvelteFragment { span, attributes, fragment })
    }

    #[inline]
    pub fn svelte_head(
        &self,
        span: Span,
        attributes: Vec<'a, ElementAttribute<'a>>,
        fragment: Fragment<'a>,
    ) -> Element<'a> {
        Element::SvelteHead(SvelteHead { span, attributes, fragment })
    }

    #[inline]
    pub fn svelte_options(
        &self,
        span: Span,
        attributes: Vec<'a, ElementAttribute<'a>>,
        fragment: Fragment<'a>,
    ) -> Element<'a> {
        Element::SvelteOptionsRaw(SvelteOptionsRaw { span, attributes, fragment })
    }

    #[inline]
    pub fn svelte_self(
        &self,
        span: Span,
        attributes: Vec<'a, ElementAttribute<'a>>,
        fragment: Fragment<'a>,
    ) -> Element<'a> {
        Element::SvelteSelf(SvelteSelf { span, attributes, fragment })
    }

    #[inline]
    pub fn svelte_window(
        &self,
        span: Span,
        attributes: Vec<'a, ElementAttribute<'a>>,
        fragment: Fragment<'a>,
    ) -> Element<'a> {
        Element::SvelteWindow(SvelteWindow { span, attributes, fragment })
    }

    #[inline]
    pub fn text(&self, span: Span, raw: Atom<'a>) -> Text<'a> {
        Text { span, data: raw.clone(), raw }
    }

    #[inline]
    pub fn attribute(
        &self,
        span: Span,
        name: Atom<'a>,
        value: AttributeValue<'a>,
    ) -> Attribute<'a> {
        Attribute { span, name, value }
    }

    #[inline]
    pub fn spread_attribute(&self, span: Span, expression: Expression<'a>) -> SpreadAttribute<'a> {
        SpreadAttribute {
            span,
            expression,
            metadata: SpreadAttributeMetadata { contains_call_expression: false, dynamic: false },
        }
    }

    #[inline]
    pub fn expression_tag(&self, span: Span, expression: Expression<'a>) -> ExpressionTag<'a> {
        ExpressionTag {
            span,
            expression,
            metadata: ExpressionTagMetadata { contains_call_expression: false, dynamic: false },
        }
    }

    #[inline]
    pub fn animate_directive(
        &self,
        span: Span,
        name: Atom<'a>,
        expression: Option<Expression<'a>>,
    ) -> Directive<'a> {
        Directive::AnimateDirective(AnimateDirective { span, name, expression })
    }

    #[inline]
    pub fn bind_directive(
        &self,
        span: Span,
        name: Atom<'a>,
        expression: BindDirectiveExpression<'a>,
    ) -> Directive<'a> {
        Directive::BindDirective(BindDirective { span, name, expression })
    }

    #[inline]
    pub fn class_directive(
        &self,
        span: Span,
        name: Atom<'a>,
        expression: Expression<'a>,
    ) -> Directive<'a> {
        Directive::ClassDirective(ClassDirective {
            span,
            name,
            expression,
            metadata: ClassDirectiveMetadata { dynamic: false },
        })
    }

    #[inline]
    pub fn let_directive(
        &self,
        span: Span,
        name: Atom<'a>,
        expression: Option<LetDirectiveExpression<'a>>,
    ) -> Directive<'a> {
        Directive::LetDirective(LetDirective { span, name, expression })
    }

    #[inline]
    pub fn on_directive(
        &self,
        span: Span,
        name: Atom<'a>,
        expression: Option<Expression<'a>>,
        modifiers: Vec<'a, Atom<'a>>,
    ) -> Directive<'a> {
        Directive::OnDirective(OnDirective { span, name, expression, modifiers })
    }

    #[inline]
    pub fn style_directive(
        &self,
        span: Span,
        name: Atom<'a>,
        value: AttributeValue<'a>,
        modifiers: Vec<'a, StyleDirectiveModifier>,
    ) -> Directive<'a> {
        Directive::StyleDirective(StyleDirective {
            span,
            name,
            value,
            modifiers,
            metadata: StyleDirectiveMetadata { dynamic: false },
        })
    }

    #[inline]
    pub fn transition_directive(
        &self,
        span: Span,
        name: Atom<'a>,
        expression: Option<Expression<'a>>,
        modifiers: Vec<'a, TransitionDirectiveModifier>,
        intro: bool,
        outro: bool,
    ) -> Directive<'a> {
        Directive::TransitionDirective(TransitionDirective {
            span,
            name,
            expression,
            modifiers,
            intro,
            outro,
        })
    }

    #[inline]
    pub fn use_directive(
        &self,
        span: Span,
        name: Atom<'a>,
        expression: Option<Expression<'a>>,
    ) -> Directive<'a> {
        Directive::UseDirective(UseDirective { span, name, expression })
    }
}
