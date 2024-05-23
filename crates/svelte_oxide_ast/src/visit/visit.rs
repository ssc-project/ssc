//! Visitor Pattern
//!
//! See:
//! * [visitor pattern](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html)
//! * [rustc visitor](https://github.com/rust-lang/rust/blob/master/compiler/rustc_ast/src/visit.rs)

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

    fn visit_root(&mut self, root: &Root<'a>) {
        walk_root(self, root);
    }

    /* ----------  Fragment ---------- */

    fn visit_fragment(&mut self, fragment: &Fragment<'a>) {
        walk_fragment(self, fragment);
    }

    fn visit_fragment_node(&mut self, node: &FragmentNodeKind<'a>) {
        walk_fragment_node(self, node);
    }

    fn visit_text(&mut self, text: &Text<'a>) {
        walk_text(self, text);
    }

    /* ----------  Tag ---------- */

    fn visit_tag(&mut self, tag: &Tag<'a>) {
        walk_tag(self, tag);
    }

    fn visit_expression_tag(&mut self, expression_tag: &ExpressionTag<'a>) {
        walk_expression_tag(self, expression_tag);
    }

    fn visit_html_tag(&mut self, html_tag: &HtmlTag<'a>) {
        walk_html_tag(self, html_tag);
    }

    fn visit_const_tag(&mut self, const_tag: &ConstTag<'a>) {
        walk_const_tag(self, const_tag);
    }

    fn visit_debug_tag(&mut self, debug_tag: &DebugTag<'a>) {
        walk_debug_tag(self, debug_tag);
    }

    fn visit_render_tag(&mut self, render_tag: &RenderTag<'a>) {
        walk_render_tag(self, render_tag);
    }

    /* ----------  Element ---------- */

    fn visit_element(&mut self, element: &Element<'a>) {
        walk_element(self, element);
    }

    fn visit_component(&mut self, component: &Component<'a>) {
        walk_component(self, component);
    }

    fn visit_title_element(&mut self, title_element: &TitleElement<'a>) {
        walk_title_element(self, title_element);
    }

    fn visit_slot_element(&mut self, slot_element: &SlotElement<'a>) {
        walk_slot_element(self, slot_element);
    }

    fn visit_regular_element(&mut self, regular_element: &RegularElement<'a>) {
        walk_regular_element(self, regular_element);
    }

    fn visit_svelte_body(&mut self, svelte_body: &SvelteBody<'a>) {
        walk_svelte_body(self, svelte_body);
    }

    fn visit_svelte_component(&mut self, svelte_component: &SvelteComponent<'a>) {
        walk_svelte_component(self, svelte_component);
    }

    fn visit_svelte_document(&mut self, svelte_document: &SvelteDocument<'a>) {
        walk_svelte_document(self, svelte_document);
    }

    fn visit_svelte_element(&mut self, svelte_element: &SvelteElement<'a>) {
        walk_svelte_element(self, svelte_element);
    }

    fn visit_svelte_fragment(&mut self, svelte_fragment: &SvelteFragment<'a>) {
        walk_svelte_fragment(self, svelte_fragment);
    }

    fn visit_svelte_head(&mut self, svelte_head: &SvelteHead<'a>) {
        walk_svelte_head(self, svelte_head);
    }

    fn visit_svelte_options_raw(&mut self, svelte_options_raw: &SvelteOptionsRaw<'a>) {
        walk_svelte_options_raw(self, svelte_options_raw);
    }

    fn visit_svelte_self(&mut self, svelte_self: &SvelteSelf<'a>) {
        walk_svelte_self(self, svelte_self);
    }

    fn visit_svelte_window(&mut self, svelte_window: &SvelteWindow<'a>) {
        walk_svelte_window(self, svelte_window);
    }

    /* ----------  Block ---------- */

    fn visit_block(&mut self, block: &Block<'a>) {
        walk_block(self, block);
    }

    fn visit_each_block(&mut self, each_block: &EachBlock<'a>) {
        walk_each_block(self, each_block);
    }

    fn visit_if_block(&mut self, if_block: &IfBlock<'a>) {
        walk_if_block(self, if_block);
    }

    fn visit_await_block(&mut self, await_block: &AwaitBlock<'a>) {
        walk_await_block(self, await_block);
    }

    fn visit_key_block(&mut self, key_block: &KeyBlock<'a>) {
        walk_key_block(self, key_block);
    }

    fn visit_snippet_block(&mut self, snippet_block: &SnippetBlock<'a>) {
        walk_snippet_block(self, snippet_block);
    }
}

pub mod walk {
    use super::*;

    pub fn walk_root<'a, V: Visit<'a>>(visitor: &mut V, root: &Root<'a>) {
        let kind = AstKind::Root(visitor.alloc(root));
        visitor.enter_node(kind);

        visitor.visit_fragment(&root.fragment);
        visitor.leave_node(kind);
    }

    /* ----------  Fragment ---------- */

    pub fn walk_fragment<'a, V: Visit<'a>>(visitor: &mut V, fragment: &Fragment<'a>) {
        for node in &fragment.nodes {
            visitor.visit_fragment_node(node);
        }
    }

    pub fn walk_fragment_node<'a, V: Visit<'a>>(visitor: &mut V, node: &FragmentNodeKind<'a>) {
        match node {
            FragmentNodeKind::Text(text) => visitor.visit_text(text),
            FragmentNodeKind::Tag(tag) => visitor.visit_tag(tag),
            FragmentNodeKind::Element(element) => visitor.visit_element(element),
            FragmentNodeKind::Block(block) => visitor.visit_block(block),
        }
    }

    pub fn walk_text<'a, V: Visit<'a>>(visitor: &mut V, text: &Text<'a>) {
        let kind = AstKind::Text(visitor.alloc(text));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    /* ----------  Tag ---------- */

    pub fn walk_tag<'a, V: Visit<'a>>(visitor: &mut V, tag: &Tag<'a>) {
        match tag {
            Tag::ExpressionTag(expression_tag) => visitor.visit_expression_tag(expression_tag),
            Tag::HtmlTag(html_tag) => visitor.visit_html_tag(html_tag),
            Tag::ConstTag(const_tag) => visitor.visit_const_tag(const_tag),
            Tag::DebugTag(debug_tag) => visitor.visit_debug_tag(debug_tag),
            Tag::RenderTag(render_tag) => visitor.visit_render_tag(render_tag),
        }
    }

    pub fn walk_expression_tag<'a, V: Visit<'a>>(
        visitor: &mut V,
        expression_tag: &ExpressionTag<'a>,
    ) {
        let kind = AstKind::ExpressionTag(visitor.alloc(expression_tag));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_html_tag<'a, V: Visit<'a>>(visitor: &mut V, html_tag: &HtmlTag<'a>) {
        let kind = AstKind::HtmlTag(visitor.alloc(html_tag));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_const_tag<'a, V: Visit<'a>>(visitor: &mut V, const_tag: &ConstTag<'a>) {
        let kind = AstKind::ConstTag(visitor.alloc(const_tag));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_debug_tag<'a, V: Visit<'a>>(visitor: &mut V, debug_tag: &DebugTag<'a>) {
        let kind = AstKind::DebugTag(visitor.alloc(debug_tag));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    pub fn walk_render_tag<'a, V: Visit<'a>>(visitor: &mut V, render_tag: &RenderTag<'a>) {
        let kind = AstKind::RenderTag(visitor.alloc(render_tag));
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    /* ----------  Element ---------- */

    pub fn walk_element<'a, V: Visit<'a>>(visitor: &mut V, element: &Element<'a>) {
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

    pub fn walk_component<'a, V: Visit<'a>>(visitor: &mut V, component: &Component<'a>) {
        let kind = AstKind::Component(visitor.alloc(component));
        visitor.enter_node(kind);
        visitor.visit_fragment(&component.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_title_element<'a, V: Visit<'a>>(visitor: &mut V, title_element: &TitleElement<'a>) {
        let kind = AstKind::TitleElement(visitor.alloc(title_element));
        visitor.enter_node(kind);
        visitor.visit_fragment(&title_element.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_slot_element<'a, V: Visit<'a>>(visitor: &mut V, slot_element: &SlotElement<'a>) {
        let kind = AstKind::SlotElement(visitor.alloc(slot_element));
        visitor.enter_node(kind);
        visitor.visit_fragment(&slot_element.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_regular_element<'a, V: Visit<'a>>(
        visitor: &mut V,
        regular_element: &RegularElement<'a>,
    ) {
        let kind = AstKind::RegularElement(visitor.alloc(regular_element));
        visitor.enter_node(kind);
        visitor.visit_fragment(&regular_element.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_body<'a, V: Visit<'a>>(visitor: &mut V, svelte_body: &SvelteBody<'a>) {
        let kind = AstKind::SvelteBody(visitor.alloc(svelte_body));
        visitor.enter_node(kind);
        visitor.visit_fragment(&svelte_body.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_component<'a, V: Visit<'a>>(
        visitor: &mut V,
        svelte_component: &SvelteComponent<'a>,
    ) {
        let kind = AstKind::SvelteComponent(visitor.alloc(svelte_component));
        visitor.enter_node(kind);
        visitor.visit_fragment(&svelte_component.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_document<'a, V: Visit<'a>>(
        visitor: &mut V,
        svelte_document: &SvelteDocument<'a>,
    ) {
        let kind = AstKind::SvelteDocument(visitor.alloc(svelte_document));
        visitor.enter_node(kind);
        visitor.visit_fragment(&svelte_document.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_element<'a, V: Visit<'a>>(
        visitor: &mut V,
        svelte_element: &SvelteElement<'a>,
    ) {
        let kind = AstKind::SvelteElement(visitor.alloc(svelte_element));
        visitor.enter_node(kind);
        visitor.visit_fragment(&svelte_element.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_fragment<'a, V: Visit<'a>>(
        visitor: &mut V,
        svelte_fragment: &SvelteFragment<'a>,
    ) {
        let kind = AstKind::SvelteFragment(visitor.alloc(svelte_fragment));
        visitor.enter_node(kind);
        visitor.visit_fragment(&svelte_fragment.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_head<'a, V: Visit<'a>>(visitor: &mut V, svelte_head: &SvelteHead<'a>) {
        let kind = AstKind::SvelteHead(visitor.alloc(svelte_head));
        visitor.enter_node(kind);
        visitor.visit_fragment(&svelte_head.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_options_raw<'a, V: Visit<'a>>(
        visitor: &mut V,
        svelte_options_raw: &SvelteOptionsRaw<'a>,
    ) {
        let kind = AstKind::SvelteOptionsRaw(visitor.alloc(svelte_options_raw));
        visitor.enter_node(kind);
        visitor.visit_fragment(&svelte_options_raw.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_self<'a, V: Visit<'a>>(visitor: &mut V, svelte_self: &SvelteSelf<'a>) {
        let kind = AstKind::SvelteSelf(visitor.alloc(svelte_self));
        visitor.enter_node(kind);
        visitor.visit_fragment(&svelte_self.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_svelte_window<'a, V: Visit<'a>>(visitor: &mut V, svelte_window: &SvelteWindow<'a>) {
        let kind = AstKind::SvelteWindow(visitor.alloc(svelte_window));
        visitor.enter_node(kind);
        visitor.visit_fragment(&svelte_window.fragment);
        visitor.leave_node(kind);
    }

    /* ----------  Block ---------- */

    pub fn walk_block<'a, V: Visit<'a>>(visitor: &mut V, block: &Block<'a>) {
        match block {
            Block::EachBlock(each_block) => visitor.visit_each_block(each_block),
            Block::IfBlock(if_block) => visitor.visit_if_block(if_block),
            Block::AwaitBlock(await_block) => visitor.visit_await_block(await_block),
            Block::KeyBlock(key_block) => visitor.visit_key_block(key_block),
            Block::SnippetBlock(snippet_block) => visitor.visit_snippet_block(snippet_block),
        }
    }

    pub fn walk_each_block<'a, V: Visit<'a>>(visitor: &mut V, each_block: &EachBlock<'a>) {
        let kind = AstKind::EachBlock(visitor.alloc(each_block));
        visitor.enter_node(kind);
        visitor.visit_fragment(&each_block.body);
        if let Some(fallback) = each_block.fallback.as_ref() {
            visitor.visit_fragment(fallback);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_if_block<'a, V: Visit<'a>>(visitor: &mut V, if_block: &IfBlock<'a>) {
        let kind = AstKind::IfBlock(visitor.alloc(if_block));
        visitor.enter_node(kind);
        visitor.visit_fragment(&if_block.consequent);
        if let Some(alternate) = if_block.alternate.as_ref() {
            visitor.visit_fragment(alternate);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_await_block<'a, V: Visit<'a>>(visitor: &mut V, await_block: &AwaitBlock<'a>) {
        let kind = AstKind::AwaitBlock(visitor.alloc(await_block));
        visitor.enter_node(kind);
        if let Some(pending) = await_block.pending.as_ref() {
            visitor.visit_fragment(pending);
        }
        if let Some(then) = await_block.then.as_ref() {
            visitor.visit_fragment(then);
        }
        if let Some(catch) = await_block.catch.as_ref() {
            visitor.visit_fragment(catch);
        }
        visitor.leave_node(kind);
    }

    pub fn walk_key_block<'a, V: Visit<'a>>(visitor: &mut V, key_block: &KeyBlock<'a>) {
        let kind = AstKind::KeyBlock(visitor.alloc(key_block));
        visitor.enter_node(kind);
        visitor.visit_fragment(&key_block.fragment);
        visitor.leave_node(kind);
    }

    pub fn walk_snippet_block<'a, V: Visit<'a>>(visitor: &mut V, snippet_block: &SnippetBlock<'a>) {
        let kind = AstKind::SnippetBlock(visitor.alloc(snippet_block));
        visitor.enter_node(kind);
        visitor.visit_fragment(&snippet_block.body);
        visitor.leave_node(kind);
    }
}
