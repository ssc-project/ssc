use oxc_ast::ast::{
    BindingPattern, Expression, IdentifierReference, VariableDeclarationKind, VariableDeclarator,
};
use oxc_diagnostics::Result;
use oxc_span::{GetSpan, SourceType};

use crate::{Kind, ParserImpl};

impl<'a> ParserImpl<'a> {
    pub(crate) fn parse_js_expression(&mut self) -> Result<Expression<'a>> {
        let parser = oxc_parser::Parser::new(
            self.allocator,
            self.source_text,
            SourceType::default().with_typescript(self.ts),
        );
        let start_pos = self.lexer.source.position();
        let expression = parser.parse_expression_from_position(self.cur_token().start)?;
        self.lexer.source.set_position(unsafe {
            if expression.span().end >= self.lexer.offset() {
                start_pos.add((expression.span().end - self.lexer.offset()) as usize)
            } else {
                start_pos.sub((self.lexer.offset() - expression.span().end) as usize)
            }
        });
        self.bump_any();
        Ok(expression)
    }

    pub(crate) fn parse_js_expression_before(&mut self, kind: Kind) -> Result<Expression<'a>> {
        let mut end = self.prev_token_end;
        let checkpoint = self.checkpoint();
        while !self.at(Kind::Eof) {
            self.bump_any();
            end = self.prev_token_end;
            if self.at(kind) {
                break;
            }
        }
        self.rewind(checkpoint);
        let source_text = &self.source_text[..(end as usize)];
        let parser = oxc_parser::Parser::new(
            self.allocator,
            source_text,
            SourceType::default().with_typescript(self.ts),
        );
        let start_pos = self.lexer.source.position();
        let expression = parser.parse_expression_from_position(self.cur_token().start)?;
        self.lexer.source.set_position(unsafe {
            if expression.span().end >= self.lexer.offset() {
                start_pos.add((expression.span().end - self.lexer.offset()) as usize)
            } else {
                start_pos.sub((self.lexer.offset() - expression.span().end) as usize)
            }
        });
        self.bump_any();
        Ok(expression)
    }

    pub(crate) fn parse_js_identifier(&mut self) -> Result<IdentifierReference<'a>> {
        let parser = oxc_parser::Parser::new(
            self.allocator,
            self.source_text,
            SourceType::default().with_typescript(self.ts),
        );
        let start_pos = self.lexer.source.position();
        let identifier = parser.parse_identifier_from_position(self.cur_token().start)?;
        self.lexer.source.set_position(unsafe {
            if identifier.span.end >= self.lexer.offset() {
                start_pos.add((identifier.span.end - self.lexer.offset()) as usize)
            } else {
                start_pos.sub((self.lexer.offset() - identifier.span.end) as usize)
            }
        });
        self.bump_any();
        Ok(identifier)
    }

    pub(crate) fn parse_js_variable_declarator(
        &mut self,
        kind: VariableDeclarationKind,
    ) -> Result<VariableDeclarator<'a>> {
        let parser = oxc_parser::Parser::new(
            self.allocator,
            self.source_text,
            SourceType::default().with_typescript(self.ts),
        );
        let start_pos = self.lexer.source.position();
        let variable_declarator =
            parser.parse_variable_declarator_from_position(self.cur_token().start, kind)?;
        self.lexer.source.set_position(unsafe {
            if variable_declarator.span.end >= self.lexer.offset() {
                start_pos.add((variable_declarator.span.end - self.lexer.offset()) as usize)
            } else {
                start_pos.sub((self.lexer.offset() - variable_declarator.span.end) as usize)
            }
        });
        self.bump_any();
        Ok(variable_declarator)
    }

    pub(crate) fn parse_js_binding_pattern(&mut self) -> Result<BindingPattern<'a>> {
        let parser = oxc_parser::Parser::new(
            self.allocator,
            self.source_text,
            SourceType::default().with_typescript(self.ts),
        );
        let start_pos = self.lexer.source.position();
        let binding_pattern = parser.parse_binding_pattern_from_position(self.cur_token().start)?;
        self.lexer.source.set_position(unsafe {
            if binding_pattern.span().end >= self.lexer.offset() {
                start_pos.add((binding_pattern.span().end - self.lexer.offset()) as usize)
            } else {
                start_pos.sub((self.lexer.offset() - binding_pattern.span().end) as usize)
            }
        });
        self.bump_any();
        Ok(binding_pattern)
    }
}
