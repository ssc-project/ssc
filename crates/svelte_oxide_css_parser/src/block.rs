use oxc_diagnostics::Result;
use svelte_oxide_css_ast::ast::*;

use crate::{Kind, ParserImpl};

impl<'a> ParserImpl<'a> {
    pub(crate) fn parse_block(&mut self) -> Result<Block<'a>> {
        println!("parsing block");
        self.expect(Kind::LCurly)?;
        let mut children = self.ast.new_vec();
        let span = self.start_span();

        while !self.at(Kind::Eof) {
            if self.at(Kind::RCurly) {
                break;
            }
            let child = self.parse_block_child()?;
            children.push(child);
        }

        self.expect(Kind::RCurly)?;

        Ok(self.ast.block(self.end_span(span), children))
    }

    fn parse_block_child(&mut self) -> Result<BlockChild<'a>> {
        println!("parsing block child");
        if self.at(Kind::At) {
            return self.parse_at_rule().map(BlockChild::AtRule);
        }

        let checkpoint = self.checkpoint();
        self.parse_value()?;
        let kind = self.cur_kind();
        self.rewind(checkpoint);

        if kind == Kind::LCurly {
            self.parse_style_rule().map(BlockChild::StyleRule)
        } else {
            self.parse_declaration().map(BlockChild::Declaration)
        }
    }

    fn parse_declaration(&mut self) -> Result<Declaration<'a>> {
        let span = self.start_span();

        self.expect_without_advance(Kind::Ident)?;
        let property = self.parse_identifier()?;
        self.expect(Kind::Colon)?;
        let value = self.parse_value()?;

        if !self.at(Kind::RCurly) {
            self.expect(Kind::Semicolon)?;
        }

        Ok(self.ast.declaration(self.end_span(span), property, value))
    }
}
