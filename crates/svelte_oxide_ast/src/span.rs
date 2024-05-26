use oxc_span::{GetSpan, Span};

use crate::ast::*;

impl<'a> GetSpan for FragmentNode<'a> {
    fn span(&self) -> Span {
        match self {
            FragmentNode::Text(text) => text.span,
            FragmentNode::Tag(tag) => tag.span(),
            FragmentNode::Element(element) => element.span(),
            FragmentNode::Block(block) => block.span(),
        }
    }
}

impl<'a> GetSpan for Tag<'a> {
    fn span(&self) -> Span {
        match self {
            Tag::ExpressionTag(expression) => expression.span,
            Tag::HtmlTag(html) => html.span,
            Tag::ConstTag(const_tag) => const_tag.span,
            Tag::DebugTag(debug) => debug.span,
            Tag::RenderTag(render) => render.span,
        }
    }
}

impl<'a> GetSpan for Element<'a> {
    fn span(&self) -> Span {
        match self {
            Element::Component(component) => component.span,
            Element::TitleElement(title) => title.span,
            Element::SlotElement(slot) => slot.span,
            Element::RegularElement(regular) => regular.span,
            Element::SvelteBody(svelte_body) => svelte_body.span,
            Element::SvelteComponent(svelte_component) => svelte_component.span,
            Element::SvelteDocument(svelte_document) => svelte_document.span,
            Element::SvelteElement(svelte_element) => svelte_element.span,
            Element::SvelteFragment(svelte_fragment) => svelte_fragment.span,
            Element::SvelteHead(svelte_head) => svelte_head.span,
            Element::SvelteOptionsRaw(svelte_options_raw) => svelte_options_raw.span,
            Element::SvelteSelf(svelte_self) => svelte_self.span,
            Element::SvelteWindow(svelte_window) => svelte_window.span,
        }
    }
}

impl<'a> GetSpan for ElementAttribute<'a> {
    fn span(&self) -> Span {
        match self {
            ElementAttribute::Attribute(attribute) => attribute.span,
            ElementAttribute::Directive(directive) => directive.span(),
            ElementAttribute::SpreadAttribute(spread_attribute) => spread_attribute.span,
        }
    }
}

impl<'a> GetSpan for Block<'a> {
    fn span(&self) -> Span {
        match self {
            Block::EachBlock(each_block) => each_block.span,
            Block::IfBlock(if_block) => if_block.span,
            Block::AwaitBlock(await_block) => await_block.span,
            Block::KeyBlock(key_block) => key_block.span,
            Block::SnippetBlock(snippet_block) => snippet_block.span,
        }
    }
}

impl<'a> GetSpan for AttributeSequenceValue<'a> {
    fn span(&self) -> Span {
        match self {
            AttributeSequenceValue::Text(text) => text.span,
            AttributeSequenceValue::ExpressionTag(tag) => tag.span,
        }
    }
}

impl<'a> GetSpan for Directive<'a> {
    fn span(&self) -> Span {
        match self {
            Directive::AnimateDirective(animate_directive) => animate_directive.span,
            Directive::BindDirective(bind_directive) => bind_directive.span,
            Directive::ClassDirective(class_directive) => class_directive.span,
            Directive::LetDirective(let_directive) => let_directive.span,
            Directive::OnDirective(on_directive) => on_directive.span,
            Directive::StyleDirective(style_directive) => style_directive.span,
            Directive::TransitionDirective(transition_directive) => transition_directive.span,
            Directive::UseDirective(use_directive) => use_directive.span,
        }
    }
}
