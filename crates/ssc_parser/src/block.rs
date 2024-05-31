use oxc_ast::ast::IdentifierName;
use oxc_diagnostics::Result;
use oxc_span::Span;
use ssc_ast::ast::*;

use crate::{Kind, ParserImpl};

impl<'a> ParserImpl<'a> {
    pub(crate) fn parse_block(&mut self) -> Result<Block<'a>> {
        let span = self.start_span();
        self.expect(Kind::LCurly)?;
        self.expect(Kind::Hash)?;
        let block = if self.eat(Kind::Each) {
            let expression = self.parse_js_expression_before(Kind::As)?;
            self.expect(Kind::As)?;
            let context = self.parse_js_binding_pattern()?;
            let index = if self.eat(Kind::Comma) {
                let identifier_ref = self.parse_js_identifier()?;
                Some(IdentifierName::new(identifier_ref.span, identifier_ref.name))
            } else {
                None
            };
            let key = if self.eat(Kind::LParen) {
                let expression = self.parse_js_expression()?;
                self.expect(Kind::RParen)?;
                Some(expression)
            } else {
                None
            };
            self.expect(Kind::RCurly)?;
            let body_children = self.parse_fragment_nodes()?;
            let body = self.ast.fragment(body_children, false);
            self.expect(Kind::LCurly)?;
            let fallback = if self.eat(Kind::Colon) {
                self.expect(Kind::Else)?;
                self.expect(Kind::RCurly)?;
                let fallback_nodes = self.parse_fragment_nodes()?;
                self.expect(Kind::LCurly)?;
                Some(self.ast.fragment(fallback_nodes, false))
            } else {
                None
            };
            self.expect(Kind::Slash)?;
            self.expect(Kind::Each)?;
            self.expect(Kind::RCurly)?;

            Block::EachBlock(self.ast.each_block(
                self.end_span(span),
                expression,
                context,
                body,
                fallback,
                index,
                key,
            ))
        } else if self.eat(Kind::If) {
            Block::IfBlock(self.continue_parsing_if_block(span, false)?)
        } else if self.eat(Kind::Await) {
            let expression = self.parse_js_expression()?;
            let (value, error, pending, then, catch) = if self.eat(Kind::Then) {
                let value = if self.eat(Kind::RCurly) {
                    None
                } else {
                    let value = self.parse_js_binding_pattern()?;
                    self.expect(Kind::RCurly)?;
                    Some(value)
                };
                let then_nodes = self.parse_fragment_nodes()?;
                let then = self.ast.fragment(then_nodes, false);
                self.expect(Kind::LCurly)?;
                self.expect(Kind::Slash)?;
                self.expect(Kind::Await)?;
                self.expect(Kind::RCurly)?;
                (value, None, None, Some(then), None)
            } else if self.eat(Kind::Catch) {
                let error = if self.eat(Kind::RCurly) {
                    None
                } else {
                    let error = self.parse_js_binding_pattern()?;
                    self.expect(Kind::RCurly)?;
                    Some(error)
                };
                let catch_nodes = self.parse_fragment_nodes()?;
                let catch = self.ast.fragment(catch_nodes, false);
                self.expect(Kind::LCurly)?;
                self.expect(Kind::Slash)?;
                self.expect(Kind::Await)?;
                self.expect(Kind::RCurly)?;
                (None, error, None, None, Some(catch))
            } else {
                self.expect(Kind::RCurly)?;
                let pending_nodes = self.parse_fragment_nodes()?;
                let pending = self.ast.fragment(pending_nodes, false);
                self.expect(Kind::LCurly)?;
                let (value, error, then, catch) = if self.eat(Kind::Colon) {
                    if self.eat(Kind::Then) {
                        let value = if self.eat(Kind::RCurly) {
                            None
                        } else {
                            let value = self.parse_js_binding_pattern()?;
                            self.expect(Kind::RCurly)?;
                            Some(value)
                        };
                        let then_nodes = self.parse_fragment_nodes()?;
                        let then = self.ast.fragment(then_nodes, false);
                        self.expect(Kind::LCurly)?;
                        let (error, catch) = if self.eat(Kind::Colon) {
                            self.expect(Kind::Catch)?;
                            let error = if self.eat(Kind::RCurly) {
                                None
                            } else {
                                let error = self.parse_js_binding_pattern()?;
                                self.expect(Kind::RCurly)?;
                                Some(error)
                            };
                            let catch_nodes = self.parse_fragment_nodes()?;
                            let catch = self.ast.fragment(catch_nodes, false);
                            self.expect(Kind::LCurly)?;
                            (error, Some(catch))
                        } else {
                            (None, None)
                        };
                        (value, error, Some(then), catch)
                    } else {
                        self.expect(Kind::Catch)?;
                        let error = if self.eat(Kind::RCurly) {
                            None
                        } else {
                            let error = self.parse_js_binding_pattern()?;
                            self.expect(Kind::RCurly)?;
                            Some(error)
                        };
                        let catch_nodes = self.parse_fragment_nodes()?;
                        let catch = self.ast.fragment(catch_nodes, false);
                        self.expect(Kind::LCurly)?;
                        (None, error, None, Some(catch))
                    }
                } else {
                    (None, None, None, None)
                };
                self.expect(Kind::Slash)?;
                self.expect(Kind::Await)?;
                self.expect(Kind::RCurly)?;
                (value, error, Some(pending), then, catch)
            };
            Block::AwaitBlock(self.ast.await_block(
                self.end_span(span),
                expression,
                value,
                error,
                pending,
                then,
                catch,
            ))
        } else if self.eat(Kind::Key) {
            let expression = self.parse_js_expression()?;
            self.expect(Kind::RCurly)?;
            let nodes = self.parse_fragment_nodes()?;
            let fragment = self.ast.fragment(nodes, false);
            self.expect(Kind::LCurly)?;
            self.expect(Kind::Slash)?;
            self.expect(Kind::Key)?;
            self.expect(Kind::RCurly)?;
            Block::KeyBlock(self.ast.key_block(self.end_span(span), expression, fragment))
        } else if self.eat(Kind::Snippet) {
            let name = self.parse_js_identifier()?;
            let name = IdentifierName::new(name.span, name.name);
            self.expect(Kind::LParen)?;
            let mut parameters = self.ast.new_vec();
            while !self.at(Kind::Eof) {
                if self.at(Kind::RParen) {
                    break;
                }
                let parameter = self.parse_js_binding_pattern()?;
                parameters.push(parameter);
            }
            self.expect(Kind::RParen)?;
            self.expect(Kind::RCurly)?;
            let nodes = self.parse_fragment_nodes()?;
            let body = self.ast.fragment(nodes, false);
            self.expect(Kind::LCurly)?;
            self.expect(Kind::Slash)?;
            self.expect(Kind::Snippet)?;
            self.expect(Kind::RCurly)?;
            Block::SnippetBlock(self.ast.snippet_block(self.end_span(span), name, parameters, body))
        } else {
            return Err(self.unexpected());
        };

        Ok(block)
    }

    // after `{#if` or `{:else if`
    fn continue_parsing_if_block(&mut self, span: Span, elseif: bool) -> Result<IfBlock<'a>> {
        let test = self.parse_js_expression()?;
        self.expect(Kind::RCurly)?;
        let consequent_nodes = self.parse_fragment_nodes()?;
        let consequent = self.ast.fragment(consequent_nodes, false);
        let alternate_span = self.start_span();
        self.expect(Kind::LCurly)?;

        let alternate = if self.eat(Kind::Colon) {
            self.expect(Kind::Else)?;
            let fragment = if self.eat(Kind::If) {
                let elseif = self.continue_parsing_if_block(alternate_span, true)?;
                let nodes = self.ast.new_vec_single(FragmentNode::Block(Block::IfBlock(elseif)));
                self.ast.fragment(nodes, false)
            } else {
                self.expect(Kind::RCurly)?;
                let nodes = self.parse_fragment_nodes()?;
                self.expect(Kind::LCurly)?;
                self.expect(Kind::Slash)?;
                self.expect(Kind::If)?;
                self.expect(Kind::RCurly)?;
                self.ast.fragment(nodes, false)
            };
            Some(fragment)
        } else {
            self.expect(Kind::Slash)?;
            self.expect(Kind::If)?;
            self.expect(Kind::RCurly)?;
            None
        };

        Ok(self.ast.if_block(self.end_span(span), elseif, test, consequent, alternate))
    }
}
