use oxc_diagnostics::Result;
use oxc_span::Atom;

use crate::{Kind, ParserImpl};

impl<'a> ParserImpl<'a> {
    pub(crate) fn parse_identifier(&mut self) -> Result<Atom<'a>> {
        let start = self.cur_token().start;
        if !self.eat(Kind::Ident)
            && !self.eat(Kind::As)
            && !self.eat(Kind::Await)
            && !self.eat(Kind::Catch)
            && !self.eat(Kind::Const)
            && !self.eat(Kind::Debug)
            && !self.eat(Kind::Each)
            && !self.eat(Kind::Else)
            && !self.eat(Kind::Html)
            && !self.eat(Kind::If)
            && !self.eat(Kind::Key)
            && !self.eat(Kind::Render)
            && !self.eat(Kind::Script)
            && !self.eat(Kind::Snippet)
            && !self.eat(Kind::Style)
            && !self.eat(Kind::Then)
        {
            return Err(self.unexpected());
        }

        while !self.at(Kind::Eof) {
            if self.prev_token_end == self.cur_token().start
                && (self.eat(Kind::Ident)
                    || self.eat(Kind::As)
                    || self.eat(Kind::Await)
                    || self.eat(Kind::Catch)
                    || self.eat(Kind::Const)
                    || self.eat(Kind::Debug)
                    || self.eat(Kind::Each)
                    || self.eat(Kind::Else)
                    || self.eat(Kind::Html)
                    || self.eat(Kind::If)
                    || self.eat(Kind::Key)
                    || self.eat(Kind::Render)
                    || self.eat(Kind::Script)
                    || self.eat(Kind::Snippet)
                    || self.eat(Kind::Style)
                    || self.eat(Kind::Then)
                    || self.eat(Kind::Minus)
                    || self.eat(Kind::Colon)
                    || self.eat(Kind::Pipe))
            {
                continue;
            }
            let ident = &self.source_text[(start as usize)..(self.prev_token_end as usize)];
            return Ok(Atom::from(ident));
        }

        Err(self.unexpected())
    }
}
