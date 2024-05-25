use oxc_diagnostics::Result;
use oxc_span::Atom;

use crate::{Kind, ParserImpl};

impl<'a> ParserImpl<'a> {
    pub(crate) fn parse_value(&mut self) -> Result<Atom<'a>> {
        let mut in_url = false;
        let span = self.start_span();

        while !self.at(Kind::Eof) {
            if in_url {
                if self.eat(Kind::RParen) {
                    in_url = false;
                } else {
                    self.eat(self.cur_kind());
                }
            } else if self.at(Kind::Semicolon) || self.at(Kind::LCurly) || self.at(Kind::RCurly) {
                let span = self.end_span(span);
                return Ok(Atom::from(
                    &self.source_text[(span.start as usize)..(span.end as usize)],
                ));
            } else {
                self.eat(self.cur_kind());
            }
        }

        Err(self.unexpected())
    }

    pub(crate) fn parse_identifier(&mut self) -> Result<Atom<'a>> {
        let span = self.start_span();

        while !self.at(Kind::Eof) {
            if self.eat(Kind::Ident) || self.eat(Kind::Minus) {
                continue;
            } else {
                let span = self.end_span(span);
                return Ok(Atom::from(
                    &self.source_text[(span.start as usize)..(span.end as usize)],
                ));
            }
        }
        Err(self.unexpected())
    }
}
