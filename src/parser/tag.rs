use lazy_static::lazy_static;
use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{ChainElement, Expression, IdentifierName};
use oxc_span::{GetSpan, SourceType, Span};
use regex::Regex;

use super::{
    errors::parse::{InvalidDebug, InvalidRenderExpression, UnexpectedToken},
    patterns::REGEX_WHITESPACE,
    Parser,
};
use crate::{
    ast::template::{
        ConstTag, DebugTag, ExpressionTag, ExpressionTagMetadata, HtmlTag,
        RenderTag, RenderTagExpression, Tag,
    },
    oxc_parser::js::declaration::{
        VariableDeclarationContext, VariableDeclarationParent,
    },
};

lazy_static! {
    static ref REGEX_WHITESPACE_WITH_CLOSING_CURLY_BRACE: Regex =
        Regex::new(r"^\s*}").unwrap();
}

impl<'a> Parser<'a> {
    pub fn parse_tag(&mut self) -> Option<Tag<'a>> {
        let start = self.index;
        if !self.eat("{", false) {
            return None;
        }

        if self.match_str("#") || self.match_str("/") || self.match_str(":") {
            self.index = start;
            return None;
        }

        if self.eat("@", false) {
            return self.parse_special_tag();
        }

        let expression = self.parse_expression();

        self.allow_whitespace();
        self.eat("}", true);

        let Some(expression) = expression else {
            // TODO: report error
            return None;
        };

        Some(Tag::Expression(ExpressionTag {
            span: Span::new(start as u32, self.index as u32),
            expression,
            metadata: ExpressionTagMetadata {
                contains_call_expression: false,
                dynamic: false,
            },
        }))
    }

    fn parse_special_tag(&mut self) -> Option<Tag<'a>> {
        let mut start = self.index;
        while &self.source_text[start..(start + 1)] != "{" {
            start -= 1;
        }

        if self.eat("html", false) {
            self.require_whitespace();

            let expression = self.parse_expression();

            self.allow_whitespace();
            self.eat("}", true);

            if let Some(expression) = expression {
                Some(Tag::Html(HtmlTag {
                    span: Span::new(start as u32, self.index as u32),
                    expression,
                }))
            } else {
                // TODO: report error
                None
            }
        } else if self.eat("debug", false) {
            let mut identifiers = OxcVec::new_in(self.allocator);

            if self.read(&REGEX_WHITESPACE_WITH_CLOSING_CURLY_BRACE).is_none() {
                let expression = self.parse_expression();

                if let Some(expression) = expression {
                    match expression {
                        Expression::Identifier(identifier) => {
                            identifiers.push(IdentifierName::new(
                                identifier.span,
                                identifier.unbox().name,
                            ));
                        }
                        Expression::SequenceExpression(sequence) => {
                            for expression in sequence.unbox().expressions {
                                if let Expression::Identifier(identifier) =
                                    expression
                                {
                                    identifiers.push(IdentifierName::new(
                                        identifier.span,
                                        identifier.unbox().name,
                                    ));
                                } else {
                                    self.error(InvalidDebug(expression.span()))
                                }
                            }
                        }
                        _ => self.error(InvalidDebug(expression.span())),
                    }
                } else {
                    // TODO: report error
                }
            }

            Some(Tag::Debug(DebugTag {
                span: Span::new(start as u32, self.index as u32),
                identifiers,
            }))
        } else if self.eat("const", false) {
            self.allow_whitespace();

            let parser = crate::oxc_parser::Parser::new(
                self.allocator,
                self.source_text,
                SourceType::default().with_typescript(self.ts),
            );

            let declaration = parser.parse_variable_declaration_at(
                start + 2,
                VariableDeclarationContext::new(
                    VariableDeclarationParent::Clause,
                ),
            );

            let declaration = match declaration {
                Ok(declaration) => declaration.unbox(),
                Err(error) => {
                    self.error(error);
                    return None;
                }
            };

            self.index = declaration.span.end as usize;
            self.allow_whitespace();
            self.eat("}", true);

            Some(Tag::Const(ConstTag {
                span: Span::new(start as u32, self.index as u32),
                declaration,
            }))
        } else if self.eat("render", false) {
            self.require_whitespace();
            let Some(expression) = self.parse_expression() else {
                self.error(InvalidRenderExpression(Span::new(
                    start as u32,
                    self.index as u32,
                )));
                return None;
            };

            let expression = match expression {
                Expression::CallExpression(call) => {
                    RenderTagExpression::Call(call.unbox())
                }
                Expression::ChainExpression(chain) => {
                    if let ChainElement::CallExpression(call) =
                        chain.unbox().expression
                    {
                        RenderTagExpression::Chain(call.unbox())
                    } else {
                        self.error(InvalidRenderExpression(Span::new(
                            start as u32,
                            self.index as u32,
                        )));
                        return None;
                    }
                }
                _ => {
                    self.error(InvalidRenderExpression(Span::new(
                        start as u32,
                        self.index as u32,
                    )));
                    return None;
                }
            };

            Some(Tag::Render(RenderTag {
                span: Span::new(start as u32, self.index as u32),
                expression,
            }))
        } else {
            // TODO: report error
            None
        }
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
