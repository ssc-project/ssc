use oxc_allocator::Box;
#[allow(clippy::wildcard_imports)]
use svelte_oxide_css_ast::ast::*;

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

impl<'a, const MINIFY: bool> Gen<MINIFY> for StyleSheet<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        for child in &self.children {
            child.gen(p);
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for Rule<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        match self {
            Self::AtRule(rule) => rule.gen(p),
            Self::StyleRule(rule) => rule.gen(p),
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for AtRule<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_indent();
        p.print(b'@');
        p.print_str(self.name.as_bytes());
        p.print_hard_space();
        p.print_str(self.prelude.as_bytes());
        if let Some(block) = self.block.as_ref() {
            p.print_soft_space();
            block.gen(p);
        } else {
            p.print_semicolon();
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for StyleRule<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_indent();
        self.prelude.gen(p);
        p.print_soft_space();
        self.block.gen(p);
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for Block<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.indent();
        p.print(b'{');
        p.print_soft_newline();
        for child in self.children.iter() {
            child.gen(p);
            p.print_soft_newline();
        }
        p.print(b'}');
        p.dedent();
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for BlockChild<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        match self {
            Self::Declaration(decl) => decl.gen(p),
            Self::StyleRule(rule) => rule.gen(p),
            Self::AtRule(rule) => rule.gen(p),
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for Declaration<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_indent();
        p.print_str(self.property.as_bytes());
        p.print(b':');
        p.print_soft_space();
        p.print_str(self.value.as_bytes());
        p.print_semicolon();
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for SelectorList<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        let len = self.children.len();
        for (i, selector) in self.children.iter().enumerate() {
            selector.gen(p);
            if (i + 1) != len {
                p.print(b',');
                p.print_soft_space();
            }
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for ComplexSelector<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        for selector in self.children.iter() {
            selector.gen(p);
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for RelativeSelector<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        if let Some(combinator) = self.combinator.as_ref() {
            if combinator.kind != CombinatorKind::Descendant {
                p.print_soft_space();
                combinator.gen(p);
                p.print_soft_space();
            } else {
                p.print_hard_space();
            }
        }

        for selector in self.selectors.iter() {
            selector.gen(p);
        }
    }
}

impl<const MINIFY: bool> Gen<MINIFY> for Combinator {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        let bytes = match self.kind {
            CombinatorKind::NextSibling => "+",
            CombinatorKind::LaterSibling => "~",
            CombinatorKind::Child => ">",
            CombinatorKind::Column => "||",
            CombinatorKind::Descendant => {
                return;
            }
        }
        .as_bytes();
        p.print_str(bytes);
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for SimpleSelector<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        match self {
            Self::TypeSelector(selector) => selector.gen(p),
            Self::IdSelector(selector) => selector.gen(p),
            Self::ClassSelector(selector) => selector.gen(p),
            Self::AttributeSelector(selector) => selector.gen(p),
            Self::PseudoElementSelector(selector) => selector.gen(p),
            Self::PseudoClassSelector(selector) => selector.gen(p),
            Self::PercentageSelector(selector) => selector.gen(p),
            Self::NthSelector(selector) => selector.gen(p),
            Self::NestingSelector(selector) => selector.gen(p),
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for TypeSelector<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(self.name.as_bytes());
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for IdSelector<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print(b'#');
        p.print_str(self.name.as_bytes());
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for ClassSelector<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print(b'.');
        p.print_str(self.name.as_bytes());
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for AttributeSelector<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print(b'[');
        p.print_str(self.name.as_bytes());
        if let Some(matcher) = self.matcher.as_ref() {
            let bytes = match matcher {
                AttributeMatcher::Substring => "~=",
                AttributeMatcher::Prefix => "^=",
                AttributeMatcher::Suffix => "$=",
                AttributeMatcher::Includes => "*=",
                AttributeMatcher::DashMatch => "|=",
                AttributeMatcher::Equal => "=",
            }
            .as_bytes();
            p.print_str(bytes);
        }

        if let Some(value) = self.value.as_ref() {
            let quoted = value.as_str().contains(' ')
                || matches!(value.as_str().chars().next(), Some('0'..='9'));
            if quoted {
                p.print(b'"');
            }
            p.print_str(value.as_bytes());

            if quoted {
                p.print(b'"');
            }

            if let Some(flags) = self.flags.as_ref() {
                p.print_hard_space();
                p.print_str(flags.as_bytes());
            }
        }

        p.print(b']');
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for PseudoElementSelector<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(b"::");
        p.print_str(self.name.as_bytes());
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for PseudoClassSelector<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print(b':');
        p.print_str(self.name.as_bytes());
        if let Some(selector) = self.args.as_ref() {
            p.print(b'(');
            selector.gen(p);
            p.print(b')');
        }
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for PercentageSelector<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(self.value.as_bytes());
    }
}

impl<'a, const MINIFY: bool> Gen<MINIFY> for NthSelector<'a> {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print_str(self.value.as_bytes());
    }
}

impl<const MINIFY: bool> Gen<MINIFY> for NestingSelector {
    fn gen(&self, p: &mut Codegen<{ MINIFY }>) {
        p.add_source_mapping(self.span.start);
        p.print(b'&');
    }
}
