use oxc_span::{Atom, Span};
use ssc_ast::ast::*;

use crate::{Kind, ParserImpl};

impl<'a> ParserImpl<'a> {
    pub(crate) fn parse_text(&mut self) -> Text<'a> {
        let start = if self.source_text[(self.prev_token_end as usize)..]
            .trim_start()
            .starts_with("<!--")
        {
            self.lexer.last_comment_end
        } else {
            self.prev_token_end
        };

        loop {
            if self.at(Kind::LCurly) || self.at(Kind::LAngle) || self.at(Kind::Eof) {
                let end = self.cur_token().start;
                self.prev_token_end = end;
                let text = &self.source_text[(start as usize)..(end as usize)];
                return self.ast.text(Span::new(start, end), Atom::from(text));
            }
            self.bump_any();
        }
    }
}
