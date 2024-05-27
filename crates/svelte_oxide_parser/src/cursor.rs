//! Code related to navigating `Token`s from the lexer

use oxc_diagnostics::Result;
use oxc_span::Span;

use crate::{
    diagnostics,
    lexer::{Kind, LexerCheckpoint, Token},
    ParserImpl,
};

#[derive(Clone, Copy)]
pub struct ParserCheckpoint<'a> {
    lexer: LexerCheckpoint<'a>,
    cur_token: Token,
    prev_span_end: u32,
    errors_pos: usize,
}

impl<'a> ParserImpl<'a> {
    pub(crate) fn start_span(&self) -> Span {
        let token = self.cur_token();
        Span::new(token.start, 0)
    }

    #[inline]
    pub(crate) fn end_span(&self, mut span: Span) -> Span {
        span.end = self.prev_token_end;
        debug_assert!(span.end >= span.start);
        span
    }

    /// Get current token
    pub(crate) fn cur_token(&self) -> Token {
        self.token
    }

    /// Get current Kind
    pub(crate) fn cur_kind(&self) -> Kind {
        self.token.kind
    }

    /// Get current string
    pub(crate) fn cur_string(&self) -> &'a str {
        self.lexer.get_string(self.token)
    }

    /// Peek next token, returns EOF for final peek
    pub(crate) fn peek_token(&mut self) -> Token {
        self.lexer.lookahead(1)
    }

    /// Peek at kind
    pub(crate) fn peek_at(&mut self, kind: Kind) -> bool {
        self.peek_token().kind == kind
    }

    /// Peek nth token
    pub(crate) fn nth(&mut self, n: u8) -> Token {
        if n == 0 {
            return self.cur_token();
        }
        self.lexer.lookahead(n)
    }

    /// Peek at nth kind
    pub(crate) fn nth_at(&mut self, n: u8, kind: Kind) -> bool {
        self.nth(n).kind == kind
    }

    /// Checks if the current index has token `Kind`
    pub(crate) fn at(&self, kind: Kind) -> bool {
        self.cur_kind() == kind
    }

    /// Move to the next token
    /// Checks if the current token is escaped if it is a keyword
    fn advance(&mut self) {
        self.prev_token_end = self.token.end;
        self.token = self.lexer.next_token();
    }

    /// Advance and return true if we are at `Kind`, return false otherwise
    pub(crate) fn eat(&mut self, kind: Kind) -> bool {
        if self.at(kind) {
            self.advance();
            return true;
        }
        false
    }

    /// Advance any token
    pub(crate) fn bump_any(&mut self) {
        self.advance();
    }

    /// # Errors
    pub(crate) fn expect_without_advance(&mut self, kind: Kind) -> Result<()> {
        if !self.at(kind) {
            let range = self.cur_token().span();
            return Err(diagnostics::expect_token(kind.to_str(), self.cur_kind().to_str(), range));
        }
        Ok(())
    }

    /// Expect a `Kind` or return error
    /// # Errors
    pub(crate) fn expect(&mut self, kind: Kind) -> Result<()> {
        self.expect_without_advance(kind)?;
        self.advance();
        Ok(())
    }

    pub(crate) fn checkpoint(&self) -> ParserCheckpoint<'a> {
        ParserCheckpoint {
            lexer: self.lexer.checkpoint(),
            cur_token: self.token,
            prev_span_end: self.prev_token_end,
            errors_pos: self.errors.len(),
        }
    }

    pub(crate) fn rewind(&mut self, checkpoint: ParserCheckpoint<'a>) {
        let ParserCheckpoint { lexer, cur_token, prev_span_end, errors_pos: errors_lens } =
            checkpoint;

        self.lexer.rewind(lexer);
        self.token = cur_token;
        self.prev_token_end = prev_span_end;
        self.errors.truncate(errors_lens);
    }
}
