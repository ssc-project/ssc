use crate::{Kind, ParserImpl};
use oxc_allocator::Vec;
use oxc_diagnostics::Result;
use svelte_oxide_ast::ast::*;

impl<'a> ParserImpl<'a> {
    pub(crate) fn parse_fragment_nodes(&mut self) -> Result<Vec<'a, FragmentNode<'a>>> {
        let mut nodes = self.ast.new_vec();

        while !self.at(Kind::Eof) {
            if self.prev_token_end != self.cur_token().start {
                let text = self.parse_text();
                nodes.push(FragmentNode::Text(text));
            } else if self.at(Kind::LAngle) {
                if self.peek_at(Kind::Slash) {
                    break;
                }

                let element = self.parse_element()?;
                nodes.push(FragmentNode::Element(element));
            } else if self.at(Kind::LCurly) {
                if self.peek_at(Kind::Colon) || self.peek_at(Kind::Slash) {
                    break;
                }
                if self.peek_at(Kind::Hash) {
                    let block = self.parse_block()?;
                    nodes.push(FragmentNode::Block(block));
                } else {
                    let tag = self.parse_tag()?;
                    nodes.push(FragmentNode::Tag(tag));
                }
            } else {
                let text = self.parse_text();
                nodes.push(FragmentNode::Text(text));
            }
        }

        Ok(nodes)
    }
}
