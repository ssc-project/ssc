use oxc_allocator::Vec;
use oxc_ast::ast::{Expression, MemberExpression, StringLiteral};
use oxc_diagnostics::Result;
use oxc_span::{Atom, GetSpan, SourceType, Span};
use svelte_oxide_ast::{ast::*, AstBuilder};

use crate::{diagnostics, Kind, ParserImpl};

macro_rules! parse_modifiers {
    ($ident: ident ($start: expr) in ($alloc: expr) {$($value: literal => $mod: expr),* $(,)?}) => {
        {
            let mut modifiers = Vec::new_in($alloc);
            let mut start = $start;
            for modifier in $ident.iter() {
                modifiers.push(match *modifier {
                    $($value => $mod),*,
                    _ => {
                        return Err(diagnostics::invalid_modifier(
                            Span::new(start, start + modifier.len() as u32),
                            modifier,
                            &[$($value),*]
                        ));
                    }
                });
                start += modifier.len() as u32 + 1;
            }
            modifiers
        }
    };
}

impl<'a> ParserImpl<'a> {
    #[allow(clippy::type_complexity)]
    pub(crate) fn parse_root_elements(
        &mut self,
    ) -> Result<(Fragment<'a>, Option<Style<'a>>, Option<Script<'a>>, Option<Script<'a>>)> {
        let mut nodes = self.ast.new_vec();
        let mut style: Option<Style<'a>> = None;
        let mut script: Option<Script<'a>> = None;
        let mut module: Option<Script<'a>> = None;

        while !self.at(Kind::Eof) {
            if self.prev_token_end != self.cur_token().start {
                let text = self.parse_text();
                nodes.push(FragmentNode::Text(text));
            } else if self.at(Kind::LAngle) {
                if self.peek_at(Kind::Script) {
                    let cur_script = self.parse_script()?;

                    if cur_script.context == ScriptContext::Default {
                        if let Some(script) = script {
                            return Err(diagnostics::duplicate_script(
                                script.span,
                                cur_script.span,
                            ));
                        } else {
                            script = Some(cur_script);
                        }
                        continue;
                    }
                    if let Some(module) = module {
                        return Err(diagnostics::duplicate_script(module.span, cur_script.span));
                    } else {
                        module = Some(cur_script);
                    }
                } else if self.peek_at(Kind::Style) {
                    let cur_style = self.parse_style()?;

                    if let Some(style) = style {
                        return Err(diagnostics::duplicate_style(style.span, cur_style.span));
                    } else {
                        style = Some(cur_style);
                    }
                } else {
                    let element = self.parse_element()?;
                    nodes.push(FragmentNode::Element(element));
                }
            } else if self.at(Kind::LCurly) {
                if self.peek_at(Kind::Hash)
                    || self.peek_at(Kind::Colon)
                    || self.peek_at(Kind::Slash)
                {
                    let block = self.parse_block()?;
                    nodes.push(FragmentNode::Block(block));
                } else {
                    let tag = self.parse_tag()?;
                    nodes.push(FragmentNode::Tag(tag));
                }
            } else {
                let text = self.parse_text();
                nodes.push(FragmentNode::Text(text));
            }
        }

        let fragment = self.ast.fragment(nodes, false);
        Ok((fragment, style, script, module))
    }

    pub(crate) fn parse_script(&mut self) -> Result<Script<'a>> {
        let span = self.start_span();
        self.expect(Kind::LAngle)?;
        self.expect(Kind::Script)?;
        let attributes = self.parse_static_attributes()?;
        self.expect(Kind::RAngle)?;
        let source_start = self.prev_token_end;
        self.parse_text();
        let source_end = loop {
            if self.at(Kind::Eof) {
                let end = self.cur_token().start;
                return Err(diagnostics::unexpected_end(Span::new(end, end)));
            } else if self.eat(Kind::LCurly) {
                self.parse_text();
            // we are at `<` if the next token isn't `/` continue
            } else if !self.peek_at(Kind::Slash) {
                self.bump_any();
                self.bump_any();
                self.parse_text();
            // we are at `</` if the next token isn't `script` continue
            } else if !self.nth_at(2, Kind::Script) {
                self.bump_any();
                self.bump_any();
                self.bump_any();
                self.parse_text();
            // at `</script`
            } else {
                break self.cur_token().start;
            }
        };
        let ret = oxc_parser::Parser::new(
            self.allocator,
            &self.source_text[..(source_end as usize)],
            SourceType::default().with_typescript(self.ts),
        )
        .parse_from_position(source_start);
        for error in ret.errors {
            self.error(error);
        }
        self.expect(Kind::LAngle)?;
        self.expect(Kind::Slash)?;
        self.expect(Kind::Script)?;
        self.expect(Kind::RAngle)?;

        Ok(self.ast.script(self.end_span(span), ScriptContext::Default, ret.program, attributes))
    }

    fn parse_style(&mut self) -> Result<Style<'a>> {
        let span = self.start_span();
        self.expect(Kind::LAngle)?;
        self.expect(Kind::Style)?;
        let attributes = self.parse_static_attributes()?;
        self.expect(Kind::RAngle)?;
        let source_start = self.prev_token_end;
        self.parse_text();
        let source_end = loop {
            if self.at(Kind::Eof) {
                let end = self.cur_token().start;
                return Err(diagnostics::unexpected_end(Span::new(end, end)));
            } else if self.eat(Kind::LCurly) {
                self.parse_text();
            // we are at `<` if the next token isn't `/` continue
            } else if !self.peek_at(Kind::Slash) {
                self.bump_any();
                self.bump_any();
                self.parse_text();
            // we are at `</` if the next token isn't `style` continue
            } else if !self.nth_at(2, Kind::Style) {
                self.bump_any();
                self.bump_any();
                self.bump_any();
                self.parse_text();
            // at `</style`
            } else {
                break self.cur_token().start;
            }
        };
        let ret = svelte_oxide_css_parser::Parser::new(
            self.allocator,
            &self.source_text[..(source_end as usize)],
        )
        .parse_from_position(source_start);
        for error in ret.errors {
            self.error(error);
        }
        self.expect(Kind::LAngle)?;
        self.expect(Kind::Slash)?;
        self.expect(Kind::Style)?;
        self.expect(Kind::RAngle)?;

        Ok(self.ast.style(
            self.end_span(span),
            ret.stylesheet,
            attributes,
            Span::new(source_start, source_end),
            Atom::from(&self.source_text[(source_start as usize)..(source_end as usize)]),
        ))
    }

    pub(crate) fn parse_element(&mut self) -> Result<Element<'a>> {
        let span = self.start_span();
        self.expect(Kind::LAngle)?;
        let name = self.parse_identifier()?;
        let attributes = self.parse_attributes()?;
        self.expect(Kind::RAngle)?;
        // this will guarantee that we are at either EOF or a closing tag
        let children = self.parse_fragment_nodes()?;
        let fragment = self.ast.fragment(children, false);
        if self.at(Kind::Eof) {
            let end = self.cur_token().start;
            return Err(diagnostics::unexpected_end(Span::new(end, end)));
        }
        let checkpoint = self.checkpoint();
        self.eat(Kind::LAngle);
        self.eat(Kind::Slash);
        let end_name = self.parse_identifier()?;
        if name.as_str() == end_name.as_str() {
            self.expect(Kind::RAngle)?;
            create_element(&self.ast, self.end_span(span), name, attributes, fragment)
        } else {
            self.rewind(checkpoint);
            create_element(&self.ast, self.end_span(span), name, attributes, fragment)
        }
    }

    fn parse_static_attributes(&mut self) -> Result<Vec<'a, Attribute<'a>>> {
        let mut attributes = self.ast.new_vec();

        while !self.at(Kind::Eof) {
            if self.at(Kind::Slash) || self.at(Kind::RAngle) {
                return Ok(attributes);
            }
            let attribute = self.parse_static_attribute()?;
            attributes.push(attribute);
        }

        let end = self.cur_token().start;
        Err(diagnostics::unexpected_end(Span::new(end, end)))
    }

    fn parse_static_attribute(&mut self) -> Result<Attribute<'a>> {
        let span = self.start_span();
        let name = self.parse_identifier()?;
        let value = if self.eat(Kind::Eq) {
            self.expect_without_advance(Kind::Str)?;
            let span = self.cur_token().span();
            let value = self.cur_string();
            AttributeValue::Sequence(self.ast.new_vec_single(AttributeSequenceValue::Text(
                self.ast.text(span, Atom::from(value)),
            )))
        } else {
            AttributeValue::Bool(true)
        };

        Ok(self.ast.attribute(self.end_span(span), name, value))
    }

    fn parse_attributes(&mut self) -> Result<Vec<'a, ElementAttribute<'a>>> {
        let mut attributes = self.ast.new_vec();

        while !self.at(Kind::Eof) {
            if self.at(Kind::Slash) || self.at(Kind::RAngle) {
                return Ok(attributes);
            }
            let attribute = self.parse_attribute()?;
            attributes.push(attribute);
        }

        let end = self.cur_token().start;
        Err(diagnostics::unexpected_end(Span::new(end, end)))
    }

    fn parse_attribute(&mut self) -> Result<ElementAttribute<'a>> {
        let span = self.start_span();
        if self.eat(Kind::LCurly) {
            if self.eat(Kind::Dot3) {
                let expression = self.parse_js_expression()?;
                self.expect(Kind::RCurly)?;
                Ok(ElementAttribute::SpreadAttribute(
                    self.ast.spread_attribute(self.end_span(span), expression),
                ))
            } else {
                let ident = self.parse_js_identifier()?;
                self.expect(Kind::RCurly)?;
                let span = self.end_span(span);
                Ok(ElementAttribute::Attribute(self.ast.attribute(
                    span,
                    ident.name.clone(),
                    AttributeValue::Sequence(self.ast.new_vec_single(
                        AttributeSequenceValue::ExpressionTag(
                            self.ast.expression_tag(
                                span,
                                Expression::Identifier(self.ast.alloc(ident)),
                            ),
                        ),
                    )),
                )))
            }
        } else {
            let name = self.parse_identifier()?;
            let (value, value_span) = if self.eat(Kind::Eq) {
                let span = self.start_span();
                let value = AttributeValue::Sequence(self.parse_attribute_value()?);
                (value, self.end_span(span))
            } else {
                let start = self.cur_token().start;
                (AttributeValue::Bool(true), Span::new(start, start))
            };

            if let Some(colon_index) = name.as_str().chars().position(|ch| ch == ':') {
                let directive_type = &name[..colon_index];
                let rest = &name[(colon_index + 1).min(name.len() - 1)..];
                let mut modifiers = rest.split('|');
                let Some(directive_name) = modifiers.next() else {
                    return Err(diagnostics::missing_directive_name(self.end_span(span)));
                };
                let modifiers: std::vec::Vec<_> = modifiers.collect();

                if directive_type == "style" {
                    let modifiers = parse_modifiers! {
                        modifiers (span.start + 2 + (directive_type.len() as u32) + (directive_name.len() as u32)) in (self.allocator) {
                            "important" => StyleDirectiveModifier::Important
                        }
                    };
                    return Ok(ElementAttribute::Directive(self.ast.style_directive(
                        self.end_span(span),
                        self.ast.new_atom(directive_name),
                        value,
                        modifiers,
                    )));
                }

                let expression = if let AttributeValue::Sequence(mut seq) = value {
                    let first = seq.remove(0);
                    let expression = if let AttributeSequenceValue::ExpressionTag(tag) = first {
                        if !seq.is_empty() {
                            return Err(diagnostics::invalid_directive_value(value_span));
                        } else {
                            tag.expression
                        }
                    } else {
                        return Err(diagnostics::invalid_directive_value(value_span));
                    };
                    Some(expression)
                } else {
                    None
                };

                if directive_type == "animate" {
                    Ok(ElementAttribute::Directive(self.ast.animate_directive(
                        self.end_span(span),
                        self.ast.new_atom(directive_name),
                        expression,
                    )))
                } else if directive_type == "bind" {
                    let expression = match expression {
                        Some(Expression::Identifier(ident)) => {
                            BindDirectiveExpression::Identifier(ident.unbox())
                        }
                        Some(Expression::ComputedMemberExpression(expr)) => {
                            BindDirectiveExpression::MemberExpression(
                                MemberExpression::ComputedMemberExpression(expr),
                            )
                        }
                        Some(Expression::StaticMemberExpression(expr)) => {
                            BindDirectiveExpression::MemberExpression(
                                MemberExpression::StaticMemberExpression(expr),
                            )
                        }
                        Some(Expression::PrivateFieldExpression(expr)) => {
                            BindDirectiveExpression::MemberExpression(
                                MemberExpression::PrivateFieldExpression(expr),
                            )
                        }
                        _ => return Err(diagnostics::invalid_bind_directive_value(value_span)),
                    };
                    Ok(ElementAttribute::Directive(self.ast.bind_directive(
                        self.end_span(span),
                        self.ast.new_atom(directive_name),
                        expression,
                    )))
                } else if directive_type == "class" {
                    let Some(expression) = expression else {
                        return Err(diagnostics::missing_class_directive_value(value_span));
                    };
                    Ok(ElementAttribute::Directive(self.ast.class_directive(
                        self.end_span(span),
                        self.ast.new_atom(directive_name),
                        expression,
                    )))
                } else if directive_type == "let" {
                    let expression = expression.map(|expression| match expression {
                        Expression::Identifier(ident) => {
                            Ok(LetDirectiveExpression::Identifier(ident.unbox()))
                        }
                        Expression::ArrayExpression(expr) => {
                            Ok(LetDirectiveExpression::ArrayExpression(expr.unbox()))
                        }
                        Expression::ObjectExpression(expr) => {
                            Ok(LetDirectiveExpression::ObjectExpression(expr.unbox()))
                        }
                        _ => Err(diagnostics::invalid_let_directive_value(value_span)),
                    });
                    let expression =
                        if let Some(expression) = expression { Some(expression?) } else { None };
                    Ok(ElementAttribute::Directive(self.ast.let_directive(
                        self.end_span(span),
                        self.ast.new_atom(directive_name),
                        expression,
                    )))
                } else if directive_type == "on" {
                    let on_directive_modifiers = self.ast.new_vec_from_iter(
                        modifiers.into_iter().map(|modifier| self.ast.new_atom(modifier)),
                    );
                    Ok(ElementAttribute::Directive(self.ast.on_directive(
                        self.end_span(span),
                        self.ast.new_atom(directive_name),
                        expression,
                        on_directive_modifiers,
                    )))
                } else if directive_type == "in"
                    || directive_type == "out"
                    || directive_type == "transition"
                {
                    let modifiers = parse_modifiers! {
                        modifiers (span.start + 2 + (directive_type.len() as u32) + (directive_name.len() as u32)) in (self.allocator) {
                            "local" => TransitionDirectiveModifier::Local,
                            "global" => TransitionDirectiveModifier::Global,
                        }
                    };

                    Ok(ElementAttribute::Directive(self.ast.transition_directive(
                        self.end_span(span),
                        self.ast.new_atom(directive_name),
                        expression,
                        modifiers,
                        directive_type == "in" || directive_name == "transition",
                        directive_type == "out" || directive_name == "transition",
                    )))
                } else if directive_type == "use" {
                    Ok(ElementAttribute::Directive(self.ast.use_directive(
                        self.end_span(span),
                        self.ast.new_atom(directive_name),
                        expression,
                    )))
                } else {
                    return Err(diagnostics::unknown_directive_type(
                        self.end_span(span),
                        directive_type,
                    ));
                }
            } else {
                Ok(ElementAttribute::Attribute(self.ast.attribute(
                    self.end_span(span),
                    name,
                    value,
                )))
            }
        }
    }

    fn parse_attribute_value(&mut self) -> Result<Vec<'a, AttributeSequenceValue<'a>>> {
        let span = self.start_span();
        if self.eat(Kind::LCurly) {
            let expression = self.parse_js_expression()?;
            self.expect(Kind::RCurly)?;
            Ok(self.ast.new_vec_single(AttributeSequenceValue::ExpressionTag(
                self.ast.expression_tag(self.end_span(span), expression),
            )))
        } else if self.at(Kind::Str) {
            let raw = self.cur_string();
            self.bump_any();
            if raw.is_empty() {
                return Ok(self.ast.new_vec_single(AttributeSequenceValue::Text(
                    self.ast.text(self.end_span(span), self.ast.new_atom(raw)),
                )));
            }
            let mut list = self.ast.new_vec();

            let mut cur_chunk_start = 0;
            let mut i = 0;

            while let Some(ch) = raw[(i as usize)..].chars().next() {
                if ch == '{' {
                    let start = i;
                    if i != cur_chunk_start {
                        list.push(AttributeSequenceValue::Text(self.ast.text(
                            Span::new(span.start + cur_chunk_start + 1, span.start + i + 1),
                            self.ast.new_atom(&raw[(cur_chunk_start as usize)..(i as usize)]),
                        )));
                    }
                    i += 1;
                    let parser = oxc_parser::Parser::new(
                        self.allocator,
                        self.source_text,
                        SourceType::default().with_typescript(self.ts),
                    );
                    let expression = parser.parse_expression_from_position(span.start + i + 1)?;
                    i = expression.span().end - span.start - 1;
                    if raw.as_bytes()[i as usize] == b'}' {
                        i += 1;
                    } else {
                        return Err(diagnostics::expect_token(
                            "}",
                            &raw.as_bytes()[i as usize].to_string(),
                            Span::new(span.start + i + 1, span.start + i + 2),
                        ));
                    }
                    cur_chunk_start = i;
                    list.push(AttributeSequenceValue::ExpressionTag(self.ast.expression_tag(
                        Span::new(span.start + start + 1, span.start + i + 1),
                        expression,
                    )));
                } else {
                    i += 1;
                }
            }

            if cur_chunk_start != i {
                list.push(AttributeSequenceValue::Text(self.ast.text(
                    Span::new(span.start + cur_chunk_start + 1, span.start + i + 1),
                    self.ast.new_atom(&raw[(cur_chunk_start as usize)..(i as usize)]),
                )));
            }

            Ok(list)
        } else {
            Err(self.unexpected())
        }
    }
}

fn create_element<'a>(
    ast: &AstBuilder<'a>,
    span: Span,
    name: Atom<'a>,
    mut attributes: Vec<'a, ElementAttribute<'a>>,
    fragment: Fragment<'a>,
) -> Result<Element<'a>> {
    Ok(match name.as_str() {
        "slot" => ast.slot_element(span, attributes, fragment),
        "title" => ast.title_element(span, attributes, fragment),
        "svelte:body" => ast.svelte_body(span, attributes, fragment),
        "svelte:component" => {
            let this_attribute_index = attributes.iter().position(|attribute| {
                if let ElementAttribute::Attribute(attribute) = attribute {
                    attribute.name.as_str() == "this"
                } else {
                    false
                }
            });
            let this_attribute = if let Some(this_attribute_index) = this_attribute_index {
                attributes.remove(this_attribute_index)
            } else {
                return Err(diagnostics::svelte_component_missing_this(span));
            };
            let this_attribute = unsafe { this_attribute.attribute().unwrap_unchecked() };
            let mut values = if let AttributeValue::Sequence(seq) = this_attribute.value {
                seq
            } else {
                return Err(diagnostics::svelte_component_invalid_this(this_attribute.span));
            };
            if values.len() != 1 {
                return Err(diagnostics::svelte_component_invalid_this(this_attribute.span));
            }
            let value = values.remove(0);
            let expression = if let AttributeSequenceValue::ExpressionTag(tag) = value {
                tag.expression
            } else {
                return Err(diagnostics::svelte_component_invalid_this(this_attribute.span));
            };
            ast.svelte_component(span, attributes, fragment, expression)
        }
        "svelte:document" => ast.svelte_document(span, attributes, fragment),
        "svelte:element" => {
            let this_attribute_index = attributes.iter().position(|attribute| {
                if let ElementAttribute::Attribute(attribute) = attribute {
                    attribute.name.as_str() == "this"
                } else {
                    false
                }
            });
            let this_attribute = if let Some(this_attribute_index) = this_attribute_index {
                attributes.remove(this_attribute_index)
            } else {
                return Err(diagnostics::svelte_element_missing_this(span));
            };
            let this_attribute = unsafe { this_attribute.attribute().unwrap_unchecked() };
            let mut values = if let AttributeValue::Sequence(seq) = this_attribute.value {
                seq
            } else {
                return Err(diagnostics::svelte_element_missing_this(span));
            };
            if values.len() != 1 {
                return Err(diagnostics::svelte_element_missing_this(span));
            }
            let value = values.remove(0);
            let expression = match value {
                AttributeSequenceValue::ExpressionTag(tag) => tag.expression,
                AttributeSequenceValue::Text(text) => {
                    Expression::StringLiteral(ast.alloc(StringLiteral::new(text.span, text.raw)))
                }
            };
            ast.svelte_element(span, attributes, fragment, expression)
        }
        "svelte:fragment" => ast.svelte_fragment(span, attributes, fragment),
        "svelte:head" => ast.svelte_head(span, attributes, fragment),
        "svelte:options" => ast.svelte_options(span, attributes, fragment),
        "svelte:self" => ast.svelte_self(span, attributes, fragment),
        "svelte:window" => ast.svelte_window(span, attributes, fragment),
        name_str => {
            if name_str.chars().next().is_some_and(|ch| ch.is_ascii_uppercase()) {
                ast.component(span, name, attributes, fragment)
            } else {
                ast.regular_element(span, name, attributes, fragment)
            }
        }
    })
}
