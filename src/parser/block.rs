use super::Parser;
use crate::ast::template::Block;

impl<'a> Parser<'a> {
    pub fn parse_block(&mut self) -> Option<Block<'a>> {
        let start = self.index;
        if !self.eat("{", false) {
            return None;
        }

        self.allow_whitespace();

        B
    }
}
