use oxc_ast::ast::{
    ChainElement, Expression, Modifiers, VariableDeclaration, VariableDeclarationKind,
};
use oxc_diagnostics::Result;
use svelte_oxide_ast::ast::*;

use crate::{diagnostics, Kind, ParserImpl};

impl<'a> ParserImpl<'a> {
    pub(crate) fn parse_tag(&mut self) -> Result<Tag<'a>> {
        let span = self.start_span();
        self.expect(Kind::LCurly)?;
        if self.eat(Kind::At) {
            let content_span = self.start_span();
            let tag = if self.eat(Kind::Html) {
                let expression = self.parse_js_expression()?;
                self.expect(Kind::RCurly)?;
                Tag::HtmlTag(self.ast.html_tag(self.end_span(span), expression))
            } else if self.eat(Kind::Const) {
                let declaration =
                    self.parse_js_variable_declarator(VariableDeclarationKind::Const)?;
                let content_span = self.end_span(content_span);
                self.expect(Kind::RCurly)?;
                Tag::ConstTag(self.ast.const_tag(
                    self.end_span(span),
                    VariableDeclaration {
                        span: content_span,
                        kind: VariableDeclarationKind::Const,
                        declarations: self.ast.new_vec_single(declaration),
                        modifiers: Modifiers::empty(),
                    },
                ))
            } else if self.eat(Kind::Debug) {
                let first_identifier = self.parse_js_identifier()?;
                let mut identifiers = self.ast.new_vec_single(first_identifier);
                while self.at(Kind::Comma) {
                    let identifier = self.parse_js_identifier()?;
                    identifiers.push(identifier);
                }
                self.expect(Kind::RCurly)?;
                Tag::DebugTag(self.ast.debug_tag(self.end_span(span), identifiers))
            } else if self.eat(Kind::Render) {
                let expression = self.parse_js_expression()?;
                self.expect(Kind::RCurly)?;
                let span = self.end_span(span);
                let expression = match expression {
                    Expression::ChainExpression(expr) => {
                        if let ChainElement::CallExpression(expr) = expr.unbox().expression {
                            RenderTagExpression::Chain(expr.unbox())
                        } else {
                            return Err(diagnostics::invalid_render_tag_expression(span));
                        }
                    }
                    Expression::CallExpression(expr) => RenderTagExpression::Call(expr.unbox()),
                    _ => {
                        return Err(diagnostics::invalid_render_tag_expression(span));
                    }
                };
                Tag::RenderTag(self.ast.render_tag(span, expression))
            } else {
                return Err(self.unexpected());
            };

            Ok(tag)
        } else {
            let expression = self.parse_js_expression()?;
            self.expect(Kind::RCurly)?;
            Ok(Tag::ExpressionTag(self.ast.expression_tag(self.end_span(span), expression)))
        }
    }
}
