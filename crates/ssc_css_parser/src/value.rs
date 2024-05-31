use oxc_diagnostics::Result;
use oxc_span::{Atom, Span};

use crate::{diagnostics, Kind, ParserImpl};

impl<'a> ParserImpl<'a> {
    pub(crate) fn parse_value(&mut self) -> Result<Atom<'a>> {
        let mut in_url = false;
        let start = self.prev_token_end;

        while !self.at(Kind::Eof) {
            if in_url {
                if self.eat(Kind::RParen) {
                    in_url = false;
                } else {
                    self.eat(self.cur_kind());
                }
            } else if self.at(Kind::Semicolon) || self.at(Kind::LCurly) || self.at(Kind::RCurly) {
                let end = self.prev_token_end;
                let starting_source = &self.source_text[(start as usize)..];
                let ending_source = &self.source_text[..(end as usize)];
                let end_trimmed = ending_source.trim_end();
                let end = if ending_source != end_trimmed {
                    let offset = (ending_source.len() - end_trimmed.len()) as u32;
                    (end - offset).max(start)
                } else {
                    end
                };
                let start_trimmed = starting_source.trim_start();
                let start = if start_trimmed != starting_source {
                    let offset = (starting_source.len() - start_trimmed.len()) as u32;
                    (start + offset).min(end)
                } else {
                    start
                };
                return Ok(Atom::from(&self.source_text[(start as usize)..(end as usize)]));
            } else {
                self.eat(self.cur_kind());
            }
        }

        let end = self.cur_token().start;
        Err(diagnostics::unexpected_end(Span::new(end, end)))
    }

    pub(crate) fn parse_identifier(&mut self) -> Result<Atom<'a>> {
        let start = self.cur_token().start;

        if !self.eat(Kind::Ident)
            && !self.eat(Kind::Minus)
            && !self.eat(Kind::Of)
            && !self.eat(Kind::Even)
            && !self.eat(Kind::Odd)
            && !self.eat(Kind::Url)
            && !self.eat(Kind::N)
        {
            return Err(self.unexpected());
        }

        while !self.at(Kind::Eof) {
            if self.prev_token_end == self.cur_token().start
                && (self.eat(Kind::Ident)
                    || self.eat(Kind::Minus)
                    || self.eat(Kind::Of)
                    || self.eat(Kind::Even)
                    || self.eat(Kind::Odd)
                    || self.eat(Kind::Url)
                    || self.eat(Kind::Number)
                    || self.eat(Kind::N))
            {
                continue;
            } else {
                let ident = &self.source_text[(start as usize)..(self.prev_token_end as usize)];
                return Ok(Atom::from(ident));
            }
        }
        let end = self.cur_token().start;
        Err(diagnostics::unexpected_end(Span::new(end, end)))
    }
}
