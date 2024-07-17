#![allow(unsafe_code)]

use oxc_ast::{
    ast::{
        BindingPattern, Expression, IdentifierReference, VariableDeclarationKind,
        VariableDeclarator,
    },
    VisitMut,
};
use oxc_diagnostics::Result;
use oxc_parser::{VariableDeclarationContext, VariableDeclarationParent};
use oxc_span::{GetSpan, SourceType};

use crate::{span_offset::SpanOffset, Kind, ParserImpl};

impl<'a> ParserImpl<'a> {
    pub(crate) fn parse_js_expression(&mut self) -> Result<Expression<'a>> {
        let span_start = self.cur_token().start;
        let mut offset = SpanOffset(span_start);
        let parser = oxc_parser::Parser::new(
            self.allocator,
            &self.source_text[span_start as usize..],
            SourceType::default().with_typescript(self.ts),
        );
        let start_pos = self.lexer.source.position();
        let mut expression = parser
            .parse_expression()
            .map_err(|mut errs| offset.transform_diagnostic(errs.remove(0)))?;
        offset.visit_expression(&mut expression);
        // SAFETY: the Oxc parser must return an expression with valid span
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
        let span_start = self.cur_token().start;
        let mut offset = SpanOffset(span_start);
        let parser = oxc_parser::Parser::new(
            self.allocator,
            &self.source_text[span_start as usize..end as usize],
            SourceType::default().with_typescript(self.ts),
        );
        let start_pos = self.lexer.source.position();
        let mut expression = parser
            .parse_expression()
            .map_err(|mut errs| offset.transform_diagnostic(errs.remove(0)))?;
        offset.visit_expression(&mut expression);
        // SAFETY: the Oxc parser must return an expression with valid span
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
        let identifier_start = self.cur_token().start;
        let mut offset = SpanOffset(identifier_start);
        let parser = oxc_parser::Parser::new(
            self.allocator,
            &self.source_text[identifier_start as usize..],
            SourceType::default().with_typescript(self.ts),
        );
        let start_pos = self.lexer.source.position();
        let mut identifier =
            parser.parse_identifier_reference().map_err(|err| offset.transform_diagnostic(err))?;
        offset.visit_identifier_reference(&mut identifier);
        // SAFETY: the Oxc parser must return an expression with valid span
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
        let span_start = self.cur_token().start;
        let mut offset = SpanOffset(span_start);
        let parser = oxc_parser::Parser::new(
            self.allocator,
            &self.source_text[span_start as usize..],
            SourceType::default().with_typescript(self.ts),
        );
        let start_pos = self.lexer.source.position();
        let mut variable_declarator = parser
            .parse_variable_declarator(
                VariableDeclarationContext::new(VariableDeclarationParent::Clause),
                kind,
            )
            .map_err(|err| offset.transform_diagnostic(err))?;
        offset.visit_variable_declarator(&mut variable_declarator);
        // SAFETY: the Oxc parser must return an expression with valid span
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
        let span_start = self.cur_token().start;
        let mut offset = SpanOffset(span_start);
        let parser = oxc_parser::Parser::new(
            self.allocator,
            &self.source_text[span_start as usize..],
            SourceType::default().with_typescript(self.ts),
        );
        let start_pos = self.lexer.source.position();
        let mut binding_pattern =
            parser.parse_binding_pattern().map_err(|err| offset.transform_diagnostic(err))?;
        offset.visit_binding_pattern(&mut binding_pattern);
        // SAFETY: the Oxc parser must return an expression with valid span
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
