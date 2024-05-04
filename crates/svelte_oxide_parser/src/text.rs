use oxc_span::{Atom, Span};
use svelte_oxide_ast::template::Text;

use crate::{utils::html::decode_character_references, Parser};

impl<'a> Parser<'a> {
    pub fn parse_text(&mut self) -> Option<Text<'a>> {
        let start = self.index;

        while self.index < self.source_text.len()
            && !self.match_str("<")
            && !self.match_str("{")
        {
            self.index += 1;
        }

        if self.index == start {
            return None;
        }

        let raw = &self.source_text[start..self.index];

        return Some(Text {
            span: Span::new(start as u32, self.index as u32),
            data: Atom::from(
                self.allocator.alloc(decode_character_references(raw, false))
                    as &str,
            ),
            raw: Atom::from(raw),
        });
    }
}
