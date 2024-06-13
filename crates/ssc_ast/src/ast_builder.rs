use std::mem;

use oxc_allocator::{Allocator, Box, String, Vec};
use oxc_ast::ast::{
    BindingPattern, Expression, IdentifierName, IdentifierReference, Program, VariableDeclaration,
};
use oxc_span::{Atom, Span};
use ssc_css_ast::ast::StyleSheet;

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
        program: Program<'a>,
        attributes: Vec<'a, Attribute<'a>>,
    ) -> Script<'a> {
        Script { span, context, program, attributes }
    }

    #[inline]
    pub fn style(
        &self,
        span: Span,
        stylesheet: StyleSheet<'a>,
        attributes: Vec<'a, Attribute<'a>>,
    ) -> Style<'a> {
        Style { span, attributes, stylesheet }
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
    pub fn animate_directive(
        &self,
        span: Span,
        name: Atom<'a>,
        expression: Option<Expression<'a>>,
    ) -> DirectiveAttribute<'a> {
        DirectiveAttribute::AnimateDirective(AnimateDirective { span, name, expression })
    }

    #[inline]
    pub fn bind_directive(
        &self,
        span: Span,
        name: Atom<'a>,
        expression: BindDirectiveExpression<'a>,
    ) -> DirectiveAttribute<'a> {
        DirectiveAttribute::BindDirective(BindDirective { span, name, expression })
    }

    #[inline]
    pub fn class_directive(
        &self,
        span: Span,
        name: Atom<'a>,
        expression: Expression<'a>,
    ) -> DirectiveAttribute<'a> {
        DirectiveAttribute::ClassDirective(ClassDirective {
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
    ) -> DirectiveAttribute<'a> {
        DirectiveAttribute::LetDirective(LetDirective { span, name, expression })
    }

    #[inline]
    pub fn on_directive(
        &self,
        span: Span,
        name: Atom<'a>,
        expression: Option<Expression<'a>>,
        modifiers: Vec<'a, Atom<'a>>,
    ) -> DirectiveAttribute<'a> {
        DirectiveAttribute::OnDirective(OnDirective { span, name, expression, modifiers })
    }

    #[inline]
    pub fn style_directive(
        &self,
        span: Span,
        name: Atom<'a>,
        value: AttributeValue<'a>,
        modifiers: Vec<'a, StyleDirectiveModifier>,
    ) -> DirectiveAttribute<'a> {
        DirectiveAttribute::StyleDirective(StyleDirective {
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
    ) -> DirectiveAttribute<'a> {
        DirectiveAttribute::TransitionDirective(TransitionDirective {
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
    ) -> DirectiveAttribute<'a> {
        DirectiveAttribute::UseDirective(UseDirective { span, name, expression })
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
    pub fn html_tag(&self, span: Span, expression: Expression<'a>) -> HtmlTag<'a> {
        HtmlTag { span, expression }
    }

    #[inline]
    pub fn const_tag(&self, span: Span, declaration: VariableDeclaration<'a>) -> ConstTag<'a> {
        ConstTag { span, declaration }
    }

    #[inline]
    pub fn debug_tag(
        &self,
        span: Span,
        identifiers: Vec<'a, IdentifierReference<'a>>,
    ) -> DebugTag<'a> {
        DebugTag { span, identifiers }
    }

    #[inline]
    pub fn render_tag(&self, span: Span, expression: RenderTagExpression<'a>) -> RenderTag<'a> {
        RenderTag { span, expression }
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn each_block(
        &self,
        span: Span,
        expression: Expression<'a>,
        context: BindingPattern<'a>,
        body: Fragment<'a>,
        fallback: Option<Fragment<'a>>,
        index: Option<IdentifierName<'a>>,
        key: Option<Expression<'a>>,
    ) -> EachBlock<'a> {
        EachBlock { span, expression, context, body, fallback, index, key }
    }

    #[inline]
    pub fn if_block(
        &self,
        span: Span,
        elseif: bool,
        test: Expression<'a>,
        consequent: Fragment<'a>,
        alternate: Option<Fragment<'a>>,
    ) -> IfBlock<'a> {
        IfBlock { span, elseif, test, consequent, alternate }
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn await_block(
        &self,
        span: Span,
        expression: Expression<'a>,
        value: Option<BindingPattern<'a>>,
        error: Option<BindingPattern<'a>>,
        pending: Option<Fragment<'a>>,
        then: Option<Fragment<'a>>,
        catch: Option<Fragment<'a>>,
    ) -> AwaitBlock<'a> {
        AwaitBlock { span, expression, value, error, pending, then, catch }
    }

    #[inline]
    pub fn key_block(
        &self,
        span: Span,
        expression: Expression<'a>,
        fragment: Fragment<'a>,
    ) -> KeyBlock<'a> {
        KeyBlock { span, expression, fragment }
    }

    pub fn snippet_block(
        &self,
        span: Span,
        expression: IdentifierName<'a>,
        parameters: Vec<'a, BindingPattern<'a>>,
        body: Fragment<'a>,
    ) -> SnippetBlock<'a> {
        SnippetBlock { span, expression, parameters, body }
    }
}
