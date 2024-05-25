use oxc_allocator::Vec;
use oxc_diagnostics::Result;
use svelte_oxide_css_ast::ast::*;

use crate::{Kind, ParserImpl};

impl<'a> ParserImpl<'a> {
    pub(crate) fn parse_rules(&mut self) -> Result<Vec<'a, Rule<'a>>> {
        let mut rules = self.ast.new_vec();

        while !self.at(Kind::Eof) {
            let rule = if self.at(Kind::At) {
                Rule::AtRule(self.parse_at_rule()?)
            } else {
                Rule::StyleRule(self.parse_style_rule()?)
            };
            rules.push(rule);
        }

        Ok(rules)
    }

    pub(crate) fn parse_at_rule(&mut self) -> Result<AtRule<'a>> {
        let span = self.start_span();
        self.expect(Kind::At)?;
        let name = self.parse_identifier()?;
        let prelude = self.parse_value()?;
        let block = if self.at(Kind::LCurly) {
            Some(self.parse_block()?)
        } else {
            self.expect(Kind::Semicolon)?;
            None
        };

        Ok(self.ast.at_rule(self.end_span(span), name, prelude, block))
    }

    pub(crate) fn parse_style_rule(&mut self) -> Result<StyleRule<'a>> {
        println!("parsing style rule");
        let span = self.start_span();
        let prelude = self.parse_selector_list(false)?;
        let block = self.parse_block()?;

        Ok(self.ast.style_rule(self.end_span(span), prelude, block))
    }
}
