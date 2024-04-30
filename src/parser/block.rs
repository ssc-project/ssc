use oxc_allocator::Vec as OxcVec;
use oxc_span::Span;

use super::{errors::parse::InvalidElseif, Parser};
use crate::ast::template::{
    Block, EachBlock, Fragment, FragmentNodeKind, IfBlock,
};

impl<'a> Parser<'a> {
    pub fn parse_block(&mut self) -> Option<Block<'a>> {
        let start = self.index;
        if !self.eat("{", false) {
            return None;
        }

        self.allow_whitespace();

        if self.eat("#", false) {
            if self.eat("if", false) {
                self.parse_if_block(false).map(Block::If)
            } else if self.eat("each", false) {
                self.parse_each_block().map(Block::Each)
            } else {
                todo!()
            }
        } else if (!self.state.is_inside_if_block)
            && (self.eat("/", false) || self.eat(":", false))
        {
            // TODO: report error
            None
        } else {
            self.index = start;
            None
        }
    }

    fn parse_if_block(&mut self, elseif: bool) -> Option<IfBlock<'a>> {
        let start = self.index - 4;
        let state_changed = if self.state.is_inside_if_block {
            false
        } else {
            self.state.is_inside_if_block = true;
            true
        };
        self.require_whitespace();
        let test = self.parse_expression();

        self.allow_whitespace();
        self.eat("}", true);

        let Some(test) = test else {
            // TODO: report error
            return None;
        };

        let consequent = self.parse_fragment(false);

        self.allow_whitespace();
        self.eat("{", true);

        let alternate = if self.eat("/", false) {
            self.eat("if", true);
            self.allow_whitespace();
            self.eat("}", true);
            None
        } else if self.eat(":", false) {
            self.eat("else", true);
            let elseif = if self.eat("if", false) {
                self.error(InvalidElseif(Span::new(
                    start as u32,
                    self.index as u32,
                )));
                true
            } else {
                self.allow_whitespace();
                self.eat("if", false)
            };

            if elseif {
                let mut nodes = OxcVec::new_in(self.allocator);
                if let Some(block) = self.parse_if_block(true) {
                    nodes.push(FragmentNodeKind::Block(Block::If(block)));
                }
                Some(Fragment { transparent: false, nodes })
            } else {
                self.allow_whitespace();
                self.eat("}", true);
                let alternate = self.parse_fragment(false);
                self.allow_whitespace();
                self.eat("{/if", true);
                self.allow_whitespace();
                self.eat("}", true);
                Some(alternate)
            }
        } else {
            None
        };

        if state_changed {
            self.state.is_inside_if_block = false;
        }

        Some(IfBlock {
            span: Span::new(start as u32, self.index as u32),
            elseif,
            test,
            consequent,
            alternate,
        })
    }

    fn parse_each_block(&mut self) -> Option<EachBlock<'a>> {
        // Some(EachBlock {
        //     span: Span::new(start as u32, self.index as u32),
        //     expression,
        //     context,
        //     body,
        //     fallback,
        // });
        todo!()
    }
}
