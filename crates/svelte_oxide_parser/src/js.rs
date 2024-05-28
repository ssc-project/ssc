use oxc_ast::ast::{Expression, IdentifierReference, VariableDeclarationKind, VariableDeclarator};
use oxc_diagnostics::Result;
use oxc_span::{GetSpan, SourceType};

use crate::ParserImpl;

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
            start_pos.add((expression.span().end - self.lexer.offset()) as usize)
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
            start_pos.add((identifier.span.end - self.lexer.offset()) as usize)
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
        let identifier =
            parser.parse_variable_declarator_from_position(self.cur_token().start, kind)?;
        self.lexer.source.set_position(unsafe {
            start_pos.add((identifier.span.end - self.lexer.offset()) as usize)
        });
        self.bump_any();
        Ok(identifier)
    }
}
