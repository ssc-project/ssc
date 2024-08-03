use oxc_allocator::Box;
use oxc_codegen::{Context, Gen as OxcGen, GenExpr};
use oxc_syntax::precedence::Precedence;
#[allow(clippy::wildcard_imports)]
use ssc_ast::ast::*;

use super::Codegen;

pub trait Gen<const MINIFY: bool> {
    fn gen(&self, _p: &mut Codegen<{ MINIFY }>) {}
}

impl<'a, const MINIFY: bool, T> Gen<MINIFY> for Box<'a, T>
where
    T: Gen<MINIFY>,
{
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        (**self).gen(p);
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for Root<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        if let Some(instance) = self.instance.as_ref() {
            instance.gen(p);
            p.print_soft_newline();
        }
        if let Some(module) = self.module.as_ref() {
            module.gen(p);
            p.print_soft_newline();
        }
        if let Some(css) = self.css.as_ref() {
            css.gen(p);
            p.print_soft_newline();
        }
        self.fragment.gen(p);
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for Script<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"<script");
        for attr in &self.attributes {
            p.print_hard_space();
            attr.gen(p);
        }
        p.print(b'>');
        let source = oxc_codegen::Codegen::<MINIFY>::new().build(&self.program).source_text;
        if !source.is_empty() {
            p.print_soft_newline();
            p.indent();
            p.print_str_with_indention(source.trim_end().as_bytes());
            p.dedent();
        }
        p.print_str(b"</script>");
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for Style<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"<style");
        for attr in &self.attributes {
            p.print_hard_space();
            attr.gen(p);
        }
        p.print(b'>');
        let options = ssc_css_codegen::CodegenOptions { enable_source_map: false };
        let source = ssc_css_codegen::Codegen::<MINIFY>::new("", "", options)
            .build(&self.stylesheet)
            .source_text;
        if !source.is_empty() {
            p.print_soft_newline();
            p.indent();
            p.print_str_with_indention(source.trim_end().as_bytes());
            p.dedent();
        }
        p.print_str(b"</style>");
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for ElementAttribute<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        match self {
            Self::Attribute(attribute) => attribute.gen(p),
            Self::SpreadAttribute(attribute) => attribute.gen(p),
            Self::DirectiveAttribute(directive) => directive.gen(p),
        };
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for Attribute<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(self.name.as_bytes());
        if let Some(value) = &self.value {
            p.print(b'=');
            value.gen(p);
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for AttributeValue<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        let tag = if self.sequence.len() == 1 {
            if let Some(AttributeSequenceValue::ExpressionTag(tag)) = self.sequence.first() {
                Some(tag)
            } else {
                None
            }
        } else {
            None
        };

        if let Some(tag) = tag {
            tag.gen(p);
        } else {
            p.print(b'"');
            for el in &self.sequence {
                match el {
                    AttributeSequenceValue::Text(text) => {
                        p.print_str(text.data.as_bytes());
                    }
                    AttributeSequenceValue::ExpressionTag(tag) => {
                        p.print(b'{');
                        let mut codegen = oxc_codegen::Codegen::<true>::new();
                        tag.expression.gen_expr(
                            &mut codegen,
                            Precedence::Lowest,
                            Context::default(),
                        );
                        let source = codegen.into_source_text();
                        p.print_str(source.as_bytes());
                        p.print(b'}');
                    }
                }
            }
            p.print(b'"');
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for SpreadAttribute<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"{...");
        print_oxc_gen_expr(&self.expression, p);
        p.print(b'}');
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for DirectiveAttribute<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        match self {
            Self::AnimateDirective(directive) => directive.gen(p),
            Self::BindDirective(directive) => directive.gen(p),
            Self::ClassDirective(directive) => directive.gen(p),
            Self::LetDirective(directive) => directive.gen(p),
            Self::OnDirective(directive) => directive.gen(p),
            Self::StyleDirective(directive) => directive.gen(p),
            Self::TransitionDirective(directive) => directive.gen(p),
            Self::UseDirective(directive) => directive.gen(p),
        };
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for AnimateDirective<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"animate:");
        p.print_str(self.name.as_bytes());
        if let Some(expression) = self.expression.as_ref() {
            p.print_str(b"={");
            print_oxc_gen_expr(expression, p);
            p.print(b'}');
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for BindDirective<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"bind:");
        p.print_str(self.name.as_bytes());
        p.print_str(b"={");
        match &self.expression {
            BindDirectiveExpression::Identifier(ident) => print_oxc_gen(ident, p),
            BindDirectiveExpression::MemberExpression(expr) => print_oxc_gen_expr(expr, p),
        };
        p.print(b'}');
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for ClassDirective<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"class:");
        p.print_str(self.name.as_bytes());
        p.print_str(b"={");
        print_oxc_gen_expr(&self.expression, p);
        p.print(b'}');
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for LetDirective<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"let:");
        p.print_str(self.name.as_bytes());
        if let Some(expression) = self.expression.as_ref() {
            p.print_str(b"={");
            match expression {
                LetDirectiveExpression::Identifier(ident) => print_oxc_gen(ident, p),
                LetDirectiveExpression::ArrayExpression(expr) => print_oxc_gen(expr, p),
                LetDirectiveExpression::ObjectExpression(expr) => print_oxc_gen_expr(expr, p),
            };
            p.print(b'}');
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for OnDirective<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"on:");
        p.print_str(self.name.as_bytes());
        for modifier in &self.modifiers {
            p.print(b'|');
            p.print_str(modifier.as_bytes());
        }
        if let Some(expression) = self.expression.as_ref() {
            p.print_str(b"={");
            print_oxc_gen_expr(expression, p);
            p.print(b'}');
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for StyleDirective<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"style:");
        p.print_str(self.name.as_bytes());
        for modifier in &self.modifiers {
            let modifier_name = match modifier {
                StyleDirectiveModifier::Important => "important",
            };
            p.print(b'|');
            p.print_str(modifier_name.as_bytes());
        }
        if let Some(value) = &self.value {
            p.print_str(b"=");
            value.gen(p);
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for TransitionDirective<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        if self.intro && !self.outro {
            p.print_str(b"in:");
        } else if self.outro && !self.intro {
            p.print_str(b"out:");
        } else {
            p.print_str(b"transition:");
        }
        p.print_str(self.name.as_bytes());
        for modifier in &self.modifiers {
            let modifier_name = match modifier {
                TransitionDirectiveModifier::Local => "local",
                TransitionDirectiveModifier::Global => "global",
            };
            p.print(b'|');
            p.print_str(modifier_name.as_bytes());
        }
        if let Some(expression) = self.expression.as_ref() {
            p.print_str(b"={");
            print_oxc_gen_expr(expression, p);
            p.print(b'}');
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for UseDirective<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"use:");
        p.print_str(self.name.as_bytes());
        if let Some(expression) = self.expression.as_ref() {
            p.print_str(b"={");
            print_oxc_gen_expr(expression, p);
            p.print(b'}');
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for Fragment<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        for node in &self.nodes {
            match node {
                FragmentNode::Text(text) => text.gen(p),
                FragmentNode::Tag(tag) => tag.gen(p),
                FragmentNode::Element(element) => element.gen(p),
                FragmentNode::Block(block) => block.gen(p),
            }
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for Text<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.print_str(self.data.as_bytes());
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for Tag<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        match self {
            Tag::ExpressionTag(tag) => tag.gen(p),
            Tag::HtmlTag(tag) => tag.gen(p),
            Tag::ConstTag(tag) => tag.gen(p),
            Tag::DebugTag(tag) => tag.gen(p),
            Tag::RenderTag(tag) => tag.gen(p),
        };
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for ExpressionTag<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print(b'{');
        print_oxc_gen_expr(&self.expression, p);
        p.print(b'}');
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for HtmlTag<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"{@html ");
        print_oxc_gen_expr(&self.expression, p);
        p.print(b'}');
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for ConstTag<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"{@");
        print_oxc_gen(&self.declaration, p);
        p.print(b'}');
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for DebugTag<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"{@debug ");
        let num_identifiers = self.identifiers.len();
        for (i, identifier) in self.identifiers.iter().enumerate() {
            print_oxc_gen(identifier, p);
            if (i + 1) != num_identifiers {
                p.print(b',');
                p.print_soft_space();
            }
        }
        p.print(b'}');
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for RenderTag<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"{@render ");
        match &self.expression {
            RenderTagExpression::Call(expr) | RenderTagExpression::Chain(expr) => {
                print_oxc_gen_expr(expr, p);
            }
        };
        p.print(b'}');
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for Element<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        match self {
            Self::Component(element) => element.gen(p),
            Self::TitleElement(element) => element.gen(p),
            Self::SlotElement(element) => element.gen(p),
            Self::RegularElement(element) => element.gen(p),
            Self::SvelteBody(element) => element.gen(p),
            Self::SvelteComponent(element) => element.gen(p),
            Self::SvelteDocument(element) => element.gen(p),
            Self::SvelteElement(element) => element.gen(p),
            Self::SvelteFragment(element) => element.gen(p),
            Self::SvelteHead(element) => element.gen(p),
            Self::SvelteOptionsRaw(element) => element.gen(p),
            Self::SvelteSelf(element) => element.gen(p),
            Self::SvelteWindow(element) => element.gen(p),
        };
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for Component<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print(b'<');
        p.print_str(self.name.as_bytes());
        for attribute in &self.attributes {
            p.print_hard_space();
            attribute.gen(p);
        }
        if self.fragment.nodes.is_empty() {
            p.print_soft_space();
            p.print_str(b"/>");
        } else {
            p.print(b'>');
            self.fragment.gen(p);
            p.print_str(b"</");
            p.print_str(self.name.as_bytes());
            p.print(b'>');
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for TitleElement<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"<title");
        for attribute in &self.attributes {
            p.print_hard_space();
            attribute.gen(p);
        }
        if self.fragment.nodes.is_empty() {
            p.print_soft_space();
            p.print_str(b"/>");
        } else {
            p.print(b'>');
            self.fragment.gen(p);
            p.print_str(b"</title>");
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for SlotElement<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"<slot");
        for attribute in &self.attributes {
            p.print_hard_space();
            attribute.gen(p);
        }
        if self.fragment.nodes.is_empty() {
            p.print_soft_space();
            p.print_str(b"/>");
        } else {
            p.print(b'>');
            self.fragment.gen(p);
            p.print_str(b"</slot>");
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for RegularElement<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print(b'<');
        p.print_str(self.name.as_bytes());
        for attribute in &self.attributes {
            p.print_hard_space();
            attribute.gen(p);
        }
        if self.fragment.nodes.is_empty() {
            p.print_soft_space();
            p.print_str(b"/>");
        } else {
            p.print(b'>');
            self.fragment.gen(p);
            p.print_str(b"</");
            p.print_str(self.name.as_bytes());
            p.print(b'>');
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for SvelteBody<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"<svelte:body");
        for attribute in &self.attributes {
            p.print_hard_space();
            attribute.gen(p);
        }
        if self.fragment.nodes.is_empty() {
            p.print_soft_space();
            p.print_str(b"/>");
        } else {
            p.print(b'>');
            self.fragment.gen(p);
            p.print_str(b"</svelte:body>");
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for SvelteComponent<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"<svelte:component this={");
        print_oxc_gen_expr(&self.expression, p);
        p.print(b'}');
        for attribute in &self.attributes {
            p.print_hard_space();
            attribute.gen(p);
        }
        if self.fragment.nodes.is_empty() {
            p.print_soft_space();
            p.print_str(b"/>");
        } else {
            p.print(b'>');
            self.fragment.gen(p);
            p.print_str(b"</svelte:component>");
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for SvelteDocument<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"<svele:document");
        for attribute in &self.attributes {
            p.print_hard_space();
            attribute.gen(p);
        }
        if self.fragment.nodes.is_empty() {
            p.print_soft_space();
            p.print_str(b"/>");
        } else {
            p.print(b'>');
            self.fragment.gen(p);
            p.print_str(b"</svelte:document>");
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for SvelteElement<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"<svelte:element this={");
        print_oxc_gen_expr(&self.expression, p);
        p.print(b'}');
        for attribute in &self.attributes {
            p.print_hard_space();
            attribute.gen(p);
        }
        if self.fragment.nodes.is_empty() {
            p.print_soft_space();
            p.print_str(b"/>");
        } else {
            p.print(b'>');
            self.fragment.gen(p);
            p.print_str(b"</svelte:element>");
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for SvelteFragment<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"<svelte:fragment");
        for attribute in &self.attributes {
            p.print_hard_space();
            attribute.gen(p);
        }
        if self.fragment.nodes.is_empty() {
            p.print_soft_space();
            p.print_str(b"/>");
        } else {
            p.print(b'>');
            self.fragment.gen(p);
            p.print_str(b"</svelte:fragment>");
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for SvelteHead<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"<svelte:head");
        for attribute in &self.attributes {
            p.print_hard_space();
            attribute.gen(p);
        }
        if self.fragment.nodes.is_empty() {
            p.print_soft_space();
            p.print_str(b"/>");
        } else {
            p.print(b'>');
            self.fragment.gen(p);
            p.print_str(b"</svelte:head>");
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for SvelteOptionsRaw<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"<svelte:options");
        for attribute in &self.attributes {
            p.print_hard_space();
            attribute.gen(p);
        }
        if self.fragment.nodes.is_empty() {
            p.print_soft_space();
            p.print_str(b"/>");
        } else {
            p.print(b'>');
            self.fragment.gen(p);
            p.print_str(b"</svelte:options>");
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for SvelteSelf<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"<svelte:self");
        for attribute in &self.attributes {
            p.print_hard_space();
            attribute.gen(p);
        }
        if self.fragment.nodes.is_empty() {
            p.print_soft_space();
            p.print_str(b"/>");
        } else {
            p.print(b'>');
            self.fragment.gen(p);
            p.print_str(b"</svelte:self>");
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for SvelteWindow<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"<svelte:window");
        for attribute in &self.attributes {
            p.print_hard_space();
            attribute.gen(p);
        }
        if self.fragment.nodes.is_empty() {
            p.print_soft_space();
            p.print_str(b"/>");
        } else {
            p.print(b'>');
            self.fragment.gen(p);
            p.print_str(b"</svelte:window>");
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for Block<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        match self {
            Self::EachBlock(block) => block.gen(p),
            Self::IfBlock(block) => block.gen(p),
            Self::AwaitBlock(block) => block.gen(p),
            Self::KeyBlock(block) => block.gen(p),
            Self::SnippetBlock(block) => block.gen(p),
        };
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for EachBlock<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"{#each ");
        print_oxc_gen_expr(&self.expression, p);
        p.print_str(b" as ");
        print_oxc_gen(&self.context, p);
        if let Some(index) = self.index.as_ref() {
            p.print(b',');
            p.print_soft_space();
            print_oxc_gen(index, p);
        }
        if let Some(key) = self.key.as_ref() {
            p.print_soft_space();
            p.print(b'(');
            print_oxc_gen_expr(key, p);
            p.print(b')');
        }
        p.print(b'}');
        self.body.gen(p);
        if let Some(fallback) = self.fallback.as_ref() {
            p.print_str(b"{:else}");
            fallback.gen(p);
        }
        p.print_str(b"{/each}");
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for IfBlock<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"{#if ");
        print_if_block(self, p);
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for AwaitBlock<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"{#await ");
        print_oxc_gen_expr(&self.expression, p);
        if let Some(pending) = self.pending.as_ref() {
            p.print(b'}');
            pending.gen(p);
            if let Some(then) = self.then.as_ref() {
                p.print_str(b"{:then");
                if let Some(value) = self.value.as_ref() {
                    p.print_hard_space();
                    print_oxc_gen(value, p);
                }
                p.print(b'}');
                then.gen(p);
            }
            if let Some(catch) = self.catch.as_ref() {
                p.print_str(b"{:catch");
                if let Some(error) = self.error.as_ref() {
                    p.print_hard_space();
                    print_oxc_gen(error, p);
                }
                p.print(b'}');
                catch.gen(p);
            }
            p.print_str(b"{/await}");
            return;
        }
        if let Some(then) = self.then.as_ref() {
            p.print_str(b" then");
            if let Some(value) = self.value.as_ref() {
                p.print_hard_space();
                print_oxc_gen(value, p);
            }
            p.print(b'}');
            then.gen(p);
            if let Some(catch) = self.catch.as_ref() {
                p.print_str(b"{:catch");
                if let Some(error) = self.error.as_ref() {
                    p.print_hard_space();
                    print_oxc_gen(error, p);
                }
                p.print(b'}');
                catch.gen(p);
            }
            p.print_str(b"{/await}");
            return;
        }
        if let Some(catch) = self.catch.as_ref() {
            p.print_str(b" catch");
            if let Some(value) = self.value.as_ref() {
                p.print_hard_space();
                print_oxc_gen(value, p);
            }
            p.print(b'}');
            catch.gen(p);
            p.print_str(b"{/await}");
            return;
        }
        p.print_str(b"}{/await}");
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for KeyBlock<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"{#key ");
        print_oxc_gen_expr(&self.expression, p);
        p.print(b'}');
        self.fragment.gen(p);
        p.print_str(b"{/key}");
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for SnippetBlock<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"{#snippet ");
        print_oxc_gen(&self.expression, p);
        p.print(b'(');
        let num_parameter = self.parameters.len();
        for (i, parameter) in self.parameters.iter().enumerate() {
            print_oxc_gen(parameter, p);
            if (i + 1) != num_parameter {
                p.print_soft_space();
                p.print(b',');
            }
        }
        p.print_str(b")}");
        self.body.gen(p);
        p.print_str(b"{/snippet}");
    }
}

fn print_if_block<const MINIFY: bool>(block: &IfBlock<'_>, p: &mut Codegen<{ MINIFY }>) {
    print_oxc_gen_expr(&block.test, p);
    p.print(b'}');
    block.consequent.gen(p);
    if let Some(alternate) = block.alternate.as_ref() {
        if alternate.nodes.len() == 1 {
            let first = &alternate.nodes[0];
            if let FragmentNode::Block(Block::IfBlock(if_block)) = first {
                if if_block.elseif {
                    p.add_source_mapping(if_block.span.start);
                    p.print_str(b"{:else if ");
                    print_if_block(if_block, p);
                    return;
                }
            }
        }
        p.print_str(b"{:else}");
        alternate.gen(p);
    }
    p.print_str(b"{/if}");
}

fn print_oxc_gen_expr<const MINIFY: bool, T: GenExpr<MINIFY>>(x: &T, p: &mut Codegen<{ MINIFY }>) {
    let mut codegen = oxc_codegen::Codegen::<MINIFY>::new();
    x.gen_expr(&mut codegen, Precedence::Lowest, Context::default());
    let source = codegen.into_source_text();
    p.print_str(source.as_bytes());
}

fn print_oxc_gen<const MINIFY: bool, T: OxcGen<MINIFY>>(x: &T, p: &mut Codegen<{ MINIFY }>) {
    let mut codegen = oxc_codegen::Codegen::<MINIFY>::new();
    x.gen(&mut codegen, Context::default());
    let source = codegen.into_source_text();
    p.print_str(source.as_bytes());
}
