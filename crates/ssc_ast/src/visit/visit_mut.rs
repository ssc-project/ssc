//! Visit Mut Pattern

use self::walk_mut::*;
use crate::{ast::*, AstType};

/// Syntax tree traversal to mutate an exclusive borrow of a syntax tree in
/// place a syntax tree in place.
pub trait VisitMut<'a>: Sized {
    fn enter_node(&mut self, _kind: AstType) {}
    fn leave_node(&mut self, _kind: AstType) {}

    fn visit_root(&mut self, root: &mut Root<'a>) {
        walk_root_mut(self, root);
    }

    /* ----------  Fragment ---------- */

    fn visit_fragment(&mut self, fragment: &mut Fragment<'a>) {
        walk_fragment_mut(self, fragment);
    }

    fn visit_fragment_node(&mut self, node: &mut FragmentNode<'a>) {
        walk_fragment_node_mut(self, node);
    }

    fn visit_text(&mut self, text: &mut Text<'a>) {
        walk_text_mut(self, text);
    }

    /* ----------  Tag ---------- */

    fn visit_tag(&mut self, tag: &mut Tag<'a>) {
        walk_tag_mut(self, tag);
    }

    fn visit_expression_tag(&mut self, expression_tag: &mut ExpressionTag<'a>) {
        walk_expression_tag_mut(self, expression_tag);
    }

    fn visit_html_tag(&mut self, html_tag: &mut HtmlTag<'a>) {
        walk_html_tag_mut(self, html_tag);
    }

    fn visit_const_tag(&mut self, const_tag: &mut ConstTag<'a>) {
        walk_const_tag_mut(self, const_tag);
    }

    fn visit_debug_tag(&mut self, debug_tag: &mut DebugTag<'a>) {
        walk_debug_tag_mut(self, debug_tag);
    }

    fn visit_render_tag(&mut self, render_tag: &mut RenderTag<'a>) {
        walk_render_tag_mut(self, render_tag);
    }

    /* ----------  Element ---------- */

    fn visit_element(&mut self, element: &mut Element<'a>) {
        walk_element_mut(self, element);
    }

    fn visit_component(&mut self, component: &mut Component<'a>) {
        walk_component_mut(self, component);
    }

    fn visit_title_element(&mut self, title_element: &mut TitleElement<'a>) {
        walk_title_element_mut(self, title_element);
    }

    fn visit_slot_element(&mut self, slot_element: &mut SlotElement<'a>) {
        walk_slot_element_mut(self, slot_element);
    }

    fn visit_regular_element(&mut self, regular_element: &mut RegularElement<'a>) {
        walk_regular_element_mut(self, regular_element);
    }

    fn visit_svelte_body(&mut self, svelte_body: &mut SvelteBody<'a>) {
        walk_svelte_body_mut(self, svelte_body);
    }

    fn visit_svelte_component(&mut self, svelte_component: &mut SvelteComponent<'a>) {
        walk_svelte_component_mut(self, svelte_component);
    }

    fn visit_svelte_document(&mut self, svelte_document: &mut SvelteDocument<'a>) {
        walk_svelte_document_mut(self, svelte_document);
    }

    fn visit_svelte_element(&mut self, svelte_element: &mut SvelteElement<'a>) {
        walk_svelte_element_mut(self, svelte_element);
    }

    fn visit_svelte_fragment(&mut self, svelte_fragment: &mut SvelteFragment<'a>) {
        walk_svelte_fragment_mut(self, svelte_fragment);
    }

    fn visit_svelte_head(&mut self, svelte_head: &mut SvelteHead<'a>) {
        walk_svelte_head_mut(self, svelte_head);
    }

    fn visit_svelte_options_raw(&mut self, svelte_options_raw: &mut SvelteOptionsRaw<'a>) {
        walk_svelte_options_raw_mut(self, svelte_options_raw);
    }

    fn visit_svelte_self(&mut self, svelte_self: &mut SvelteSelf<'a>) {
        walk_svelte_self_mut(self, svelte_self);
    }

    fn visit_svelte_window(&mut self, svelte_window: &mut SvelteWindow<'a>) {
        walk_svelte_window_mut(self, svelte_window);
    }

    /* ----------  Block ---------- */

    fn visit_block(&mut self, block: &mut Block<'a>) {
        walk_block_mut(self, block);
    }

    fn visit_each_block(&mut self, each_block: &mut EachBlock<'a>) {
        walk_each_block_mut(self, each_block);
    }

    fn visit_if_block(&mut self, if_block: &mut IfBlock<'a>) {
        walk_if_block_mut(self, if_block);
    }

    fn visit_await_block(&mut self, await_block: &mut AwaitBlock<'a>) {
        walk_await_block_mut(self, await_block);
    }

    fn visit_key_block(&mut self, key_block: &mut KeyBlock<'a>) {
        walk_key_block_mut(self, key_block);
    }

    fn visit_snippet_block(&mut self, snippet_block: &mut SnippetBlock<'a>) {
        walk_snippet_block_mut(self, snippet_block);
    }
}

pub mod walk_mut {
    use super::*;

    pub fn walk_root_mut<'a, V: VisitMut<'a>>(visitor: &mut V, root: &mut Root<'a>) {
        let kind = AstType::Root;
        visitor.enter_node(kind);

        visitor.visit_fragment(&mut root.fragment);
        visitor.leave_node(kind);
    }

    /* ----------  Fragment ---------- */

    pub fn walk_fragment_mut<'a, V: VisitMut<'a>>(visitor: &mut V, fragment: &mut Fragment<'a>) {
        for node in fragment.nodes.iter_mut() {
            visitor.visit_fragment_node(node);
        }
    }

    pub fn walk_fragment_node_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        node: &mut FragmentNode<'a>,
    ) {
        match node {
            FragmentNode::Text(text) => visitor.visit_text(text),
            FragmentNode::Tag(tag) => visitor.visit_tag(tag),
            FragmentNode::Element(element) => visitor.visit_element(element),
            FragmentNode::Block(block) => visitor.visit_block(block),
        }
    }

    pub fn walk_text_mut<'a, V: VisitMut<'a>>(visitor: &mut V, _text: &mut Text<'a>) {
        let kind = AstType::Text;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    /* ----------  Tag ---------- */

    pub fn walk_tag_mut<'a, V: VisitMut<'a>>(visitor: &mut V, tag: &mut Tag<'a>) {
        match tag {
            Tag::ExpressionTag(expression_tag) => visitor.visit_expression_tag(expression_tag),
            Tag::HtmlTag(html_tag) => visitor.visit_html_tag(html_tag),
            Tag::ConstTag(const_tag) => visitor.visit_const_tag(const_tag),
            Tag::DebugTag(debug_tag) => visitor.visit_debug_tag(debug_tag),
            Tag::RenderTag(render_tag) => visitor.visit_render_tag(render_tag),
        }
    }

    pub fn walk_expression_tag_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        _expression_tag: &mut ExpressionTag<'a>,
    ) {
        let kind = AstType::ExpressionTag;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_html_tag_mut<'a, V: VisitMut<'a>>(visitor: &mut V, _html_tag: &mut HtmlTag<'a>) {
        let kind = AstType::HtmlTag;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_const_tag_mut<'a, V: VisitMut<'a>>(visitor: &mut V, _const_tag: &mut ConstTag<'a>) {
        let kind = AstType::ConstTag;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_debug_tag_mut<'a, V: VisitMut<'a>>(visitor: &mut V, _debug_tag: &mut DebugTag<'a>) {
        let kind = AstType::DebugTag;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_render_tag_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        _render_tag: &mut RenderTag<'a>,
    ) {
        let kind = AstType::RenderTag;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    /* ----------  Element ---------- */

    pub fn walk_element_mut<'a, V: VisitMut<'a>>(visitor: &mut V, element: &mut Element<'a>) {
        match element {
            Element::Component(component) => visitor.visit_component(component),
            Element::TitleElement(title_element) => visitor.visit_title_element(title_element),
            Element::SlotElement(slot_element) => visitor.visit_slot_element(slot_element),
            Element::RegularElement(regular_element) => {
                visitor.visit_regular_element(regular_element)
            }
            Element::SvelteBody(svelte_body) => visitor.visit_svelte_body(svelte_body),
            Element::SvelteComponent(svelte_component) => {
                visitor.visit_svelte_component(svelte_component)
            }
            Element::SvelteDocument(svelte_document) => {
                visitor.visit_svelte_document(svelte_document)
            }
            Element::SvelteElement(svelte_element) => visitor.visit_svelte_element(svelte_element),
            Element::SvelteFragment(svelte_fragment) => {
                visitor.visit_svelte_fragment(svelte_fragment)
            }
            Element::SvelteHead(svelte_head) => visitor.visit_svelte_head(svelte_head),
            Element::SvelteOptionsRaw(svelte_options_raw) => {
                visitor.visit_svelte_options_raw(svelte_options_raw)
            }
            Element::SvelteSelf(svelte_self) => visitor.visit_svelte_self(svelte_self),
            Element::SvelteWindow(svelte_window) => visitor.visit_svelte_window(svelte_window),
        }
    }

    pub fn walk_component_mut<'a, V: VisitMut<'a>>(visitor: &mut V, component: &mut Component<'a>) {
        let kind = AstType::Component;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut component.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_title_element_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        title_element: &mut TitleElement<'a>,
    ) {
        let kind = AstType::TitleElement;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut title_element.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_slot_element_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        slot_element: &mut SlotElement<'a>,
    ) {
        let kind = AstType::SlotElement;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut slot_element.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_regular_element_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        regular_element: &mut RegularElement<'a>,
    ) {
        let kind = AstType::RegularElement;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut regular_element.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_body_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        svelte_body: &mut SvelteBody<'a>,
    ) {
        let kind = AstType::SvelteBody;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut svelte_body.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_component_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        svelte_component: &mut SvelteComponent<'a>,
    ) {
        let kind = AstType::SvelteComponent;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut svelte_component.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_document_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        svelte_document: &mut SvelteDocument<'a>,
    ) {
        let kind = AstType::SvelteDocument;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut svelte_document.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_element_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        svelte_element: &mut SvelteElement<'a>,
    ) {
        let kind = AstType::SvelteElement;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut svelte_element.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_fragment_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        svelte_fragment: &mut SvelteFragment<'a>,
    ) {
        let kind = AstType::SvelteFragment;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut svelte_fragment.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_head_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        svelte_head: &mut SvelteHead<'a>,
    ) {
        let kind = AstType::SvelteHead;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut svelte_head.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_options_raw_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        svelte_options_raw: &mut SvelteOptionsRaw<'a>,
    ) {
        let kind = AstType::SvelteOptionsRaw;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut svelte_options_raw.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_self_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        svelte_self: &mut SvelteSelf<'a>,
    ) {
        let kind = AstType::SvelteSelf;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut svelte_self.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_window_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        svelte_window: &mut SvelteWindow<'a>,
    ) {
        let kind = AstType::SvelteWindow;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut svelte_window.fragment);
        visitor.leave_node(kind);
    }

    /* ----------  Block ---------- */

    pub fn walk_block_mut<'a, V: VisitMut<'a>>(visitor: &mut V, block: &mut Block<'a>) {
        match block {
            Block::EachBlock(each_block) => visitor.visit_each_block(each_block),
            Block::IfBlock(if_block) => visitor.visit_if_block(if_block),
            Block::AwaitBlock(await_block) => visitor.visit_await_block(await_block),
            Block::KeyBlock(key_block) => visitor.visit_key_block(key_block),
            Block::SnippetBlock(snippet_block) => visitor.visit_snippet_block(snippet_block),
        }
    }

    pub fn walk_each_block_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        each_block: &mut EachBlock<'a>,
    ) {
        let kind = AstType::EachBlock;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut each_block.body);
        if let Some(fallback) = each_block.fallback.as_mut() {
            visitor.visit_fragment(fallback);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_if_block_mut<'a, V: VisitMut<'a>>(visitor: &mut V, if_block: &mut IfBlock<'a>) {
        let kind = AstType::IfBlock;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut if_block.consequent);
        if let Some(alternate) = if_block.alternate.as_mut() {
            visitor.visit_fragment(alternate);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_await_block_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        await_block: &mut AwaitBlock<'a>,
    ) {
        let kind = AstType::AwaitBlock;
        visitor.enter_node(kind);
        if let Some(pending) = await_block.pending.as_mut() {
            visitor.visit_fragment(pending);
        }
        if let Some(then) = await_block.then.as_mut() {
            visitor.visit_fragment(then);
        }
        if let Some(catch) = await_block.catch.as_mut() {
            visitor.visit_fragment(catch);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_key_block_mut<'a, V: VisitMut<'a>>(visitor: &mut V, key_block: &mut KeyBlock<'a>) {
        let kind = AstType::KeyBlock;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut key_block.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_snippet_block_mut<'a, V: VisitMut<'a>>(
        visitor: &mut V,
        snippet_block: &mut SnippetBlock<'a>,
    ) {
        let kind = AstType::SnippetBlock;
        visitor.enter_node(kind);
        visitor.visit_fragment(&mut snippet_block.body);
        visitor.leave_node(kind);
    }
}
