use lazy_static::lazy_static;
use oxc_span::{Atom, Span};
use regex::Regex;
use svelte_oxide_ast::template::Comment;

use crate::{utils::extract_svelte_ignore::extract_svelte_ignore, Parser};

lazy_static! {
    static ref REGEX_CLOSING_COMMENT: Regex = Regex::new("-->").unwrap();
}

impl<'a> Parser<'a> {
    pub fn parse_comment(&mut self) -> Option<Comment<'a>> {
        let start = self.index;
        if !self.eat("<!--", false) {
            return None;
        }
        let data = self.read_until(&REGEX_CLOSING_COMMENT).to_string();
        let data = self.allocator.alloc(data);
        self.eat("-->", true);
        let ignores = extract_svelte_ignore(self.allocator, data);
        Some(Comment {
            span: Span::new(start as u32, self.index as u32),
            data: Atom::from(data as &str),
            ignores,
        })
    }
}
