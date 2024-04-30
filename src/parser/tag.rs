use oxc_ast::ast::Expression;
use oxc_span::{GetSpan, SourceType, Span};

use super::{
    errors::parse::UnexpectedToken, patterns::REGEX_WHITESPACE, Parser,
};
use crate::ast::template::Tag;

impl<'a> Parser<'a> {
    pub fn parse_tag(&mut self) -> Option<Tag<'a>> {
        todo!()
    }

    pub fn parse_expression(&mut self) -> Option<Expression<'a>> {
        let parser = crate::oxc_parser::Parser::new(
            self.allocator,
            self.source_text,
            SourceType::default().with_typescript(self.ts),
        );
        match parser.parse_expression_at(self.index) {
            Ok(expression) => {
                let mut num_parens = 0;
                for i in self.index..expression.span().start as usize {
                    let ch: char = self.source_text.as_bytes()[i].into();
                    if ch == '(' {
                        num_parens += 1;
                    }
                }

                let mut index = expression.span().end as usize;

                while num_parens > 0 {
                    let ch: char = self.source_text.as_bytes()[index].into();

                    if ch == ')' {
                        num_parens -= 1;
                    } else if !REGEX_WHITESPACE.is_match(&ch.to_string()) {
                        self.error(UnexpectedToken(
                            Span::new(index as u32, index as u32 + 1),
                            ")".to_string(),
                        ));
                    }

                    index += 1;
                }

                self.index = index;

                Some(expression)
            }
            Err(error) => {
                self.error(error);
                None
            }
        }
    }
}
