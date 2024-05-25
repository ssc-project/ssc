use oxc_diagnostics::Result;
use oxc_span::{Atom, Span};
use svelte_oxide_css_ast::ast::*;

use crate::{diagnostics::invalid_css_selector, Kind, ParserImpl};

impl<'a> ParserImpl<'a> {
    pub(crate) fn parse_selector_list(
        &mut self,
        inside_pseudo_class: bool,
    ) -> Result<SelectorList<'a>> {
        let span = self.start_span();
        let mut children = self.ast.new_vec();
        let ending_kind = if inside_pseudo_class { Kind::RParen } else { Kind::LCurly };

        while !self.at(Kind::Eof) {
            let selector = self.parse_selector(inside_pseudo_class)?;
            children.push(selector);

            if self.at(ending_kind) {
                return Ok(self.ast.selector_list(self.end_span(span), children));
            }

            self.expect(Kind::Comma)?;
        }

        Err(self.unexpected())
    }

    fn parse_selector(&mut self, inside_pseudo_class: bool) -> Result<ComplexSelector<'a>> {
        let list_span = self.start_span();
        let mut children = self.ast.new_vec();
        let ending_kind = if inside_pseudo_class { Kind::RParen } else { Kind::LCurly };

        let mut relative_selector =
            self.ast.relative_selector(self.start_span(), None, self.ast.new_vec());

        while !self.at(Kind::Eof) {
            let span = self.start_span();

            // this is a workaround, because if-let chains arn't stable yet.
            'matching: {
                if self.eat(Kind::Amp) {
                    relative_selector
                        .selectors
                        .push(self.ast.nesting_selector(self.end_span(span)));
                } else if self.eat(Kind::Star) {
                    let name = if self.eat(Kind::Pipe) {
                        self.parse_identifier()?
                    } else {
                        Atom::from("*")
                    };

                    relative_selector.selectors.push(self.ast.type_selector(span, name));
                } else if self.eat(Kind::Hash) {
                    let name = self.parse_identifier()?;
                    relative_selector
                        .selectors
                        .push(self.ast.id_selector(self.end_span(span), name));
                } else if self.eat(Kind::Dot) {
                    let name = self.parse_identifier()?;
                    relative_selector
                        .selectors
                        .push(self.ast.class_selector(self.end_span(span), name));
                } else if self.eat(Kind::Colon2) {
                    let name = self.parse_identifier()?;
                    relative_selector.selectors.push(self.ast.pseudo_element_selector(span, name));

                    // We parse the inner selectors of a pseudo element to ensure it parses correctly,
                    // but we don't do anything with the result
                    if self.eat(Kind::LParen) {
                        self.parse_selector_list(true)?;
                        self.expect(Kind::RParen)?;
                    }
                } else if self.eat(Kind::Colon) {
                    let name = self.parse_identifier()?;

                    let args = if self.eat(Kind::LParen) {
                        let args = self.parse_selector_list(true)?;
                        self.expect(Kind::RParen)?;
                        Some(args)
                    } else {
                        None
                    };

                    relative_selector.selectors.push(self.ast.pseudo_class_selector(
                        self.end_span(span),
                        name,
                        args,
                    ));
                } else if self.eat(Kind::LBrack) {
                    let name = self.parse_identifier()?;
                    println!("Name: {}", name);
                    let matcher = self.parse_matcher()?;

                    println!("Matcher: {:?}", matcher);

                    let value =
                        if matcher.is_some() { Some(self.parse_attribute_value()?) } else { None };

                    println!("Value: {:?}", value);

                    let flags = self
                        .parse_identifier()
                        .ok()
                        .and_then(|flags| (!flags.as_str().is_empty()).then_some(flags));

                    self.expect(Kind::RBrack)?;

                    relative_selector.selectors.push(self.ast.attribute_selector(
                        self.end_span(span),
                        name,
                        matcher,
                        value,
                        flags,
                    ));
                } else if inside_pseudo_class {
                    if let Some(value) = self.parse_nth_selector()? {
                        relative_selector
                            .selectors
                            .push(self.ast.nth_selector(self.end_span(span), value));
                        break 'matching;
                    }
                } else if let Some(value) = self.parse_percentage_selector()? {
                    relative_selector
                        .selectors
                        .push(self.ast.percentage_selector(self.end_span(span), value));
                } else if self
                    .parse_combinator_kind(!relative_selector.selectors.is_empty())
                    .is_none()
                {
                    let name = self.parse_identifier()?;

                    let name = if self.eat(Kind::Pipe) { self.parse_identifier()? } else { name };

                    relative_selector
                        .selectors
                        .push(self.ast.type_selector(self.end_span(span), name));
                }
            }

            if self.at(Kind::Comma) || self.at(ending_kind) {
                relative_selector.span = self.end_span(relative_selector.span);

                children.push(relative_selector);

                return Ok(self.ast.complex_selector(self.end_span(list_span), children));
            }

            if let Some(combinator) = self.parse_combinator(!relative_selector.selectors.is_empty())
            {
                if relative_selector.selectors.is_empty() {
                    if combinator.kind == CombinatorKind::Descendant {
                        continue;
                    }
                    if !inside_pseudo_class {
                        return Err(invalid_css_selector(self.end_span(span)));
                    }
                } else {
                    relative_selector.span = self.end_span(relative_selector.span);
                    children.push(relative_selector);
                }

                relative_selector = self.ast.relative_selector(
                    Span::new(combinator.span.start, 0),
                    Some(combinator),
                    self.ast.new_vec(),
                );

                if self.at(Kind::Comma) || self.at(ending_kind) {
                    return Err(invalid_css_selector(self.end_span(span)));
                }
            }
        }

        Err(self.unexpected())
    }

    fn parse_matcher(&mut self) -> Result<Option<AttributeMatcher>> {
        let matcher = match self.cur_kind() {
            Kind::Tilde => AttributeMatcher::Substring,
            Kind::Caret => AttributeMatcher::Prefix,
            Kind::Dollar => AttributeMatcher::Suffix,
            Kind::Star => AttributeMatcher::Includes,
            Kind::Pipe => AttributeMatcher::DashMatch,
            Kind::Eq => {
                self.bump_any();
                return Ok(Some(AttributeMatcher::Equal));
            }
            _ => {
                return Ok(None);
            }
        };

        self.eat(self.cur_kind());
        self.expect(Kind::Eq)?;

        Ok(Some(matcher))
    }

    fn parse_attribute_value(&mut self) -> Result<Atom<'a>> {
        if self.at(Kind::Str) {
            let value = self.cur_string();
            self.bump_any();
            Ok(Atom::from(value))
        } else {
            self.parse_identifier()
        }
    }

    fn parse_nth_selector(&mut self) -> Result<Option<Atom<'a>>> {
        let span = self.start_span();
        if self.parse_nth_selector_start()?.is_none() {
            return Ok(None);
        }
        self.parse_nth_selector_end()?;
        let span = self.end_span(span);
        let selector = &self.source_text[(span.start as usize)..(span.end as usize)];

        Ok(Some(Atom::from(selector)))
    }

    fn parse_nth_selector_start(&mut self) -> Result<Option<()>> {
        if self.eat(Kind::Even) || self.eat(Kind::Odd) {
            return Ok(Some(()));
        }
        if self.eat(Kind::Minus) {
            self.eat(Kind::Number);
            self.expect(Kind::N)?;
            self.expect(Kind::Plus)?;
            self.expect(Kind::Number)?;

            return Ok(Some(()));
        }

        let started = self.eat(Kind::Plus);

        if self.eat(Kind::Number) {
            if !self.eat(Kind::N) {
                return Ok(Some(()));
            }
        } else if started {
            self.expect(Kind::N)?;
        } else if !self.eat(Kind::N) {
            return Ok(None);
        }

        if self.eat(Kind::Plus) || self.eat(Kind::Minus) {
            self.expect(Kind::Number)?;
        }

        Ok(Some(()))
    }

    fn parse_nth_selector_end(&mut self) -> Result<()> {
        if self.eat(Kind::Comma) || self.eat(Kind::RParen) || self.eat(Kind::Of) {
            return Ok(());
        }
        Err(self.unexpected())
    }

    fn parse_percentage_selector(&mut self) -> Result<Option<Atom<'a>>> {
        let span = self.start_span();
        if !self.eat(Kind::Number) {
            return Ok(None);
        }
        if self.eat(Kind::Dot) {
            self.expect(Kind::Number)?;
        }
        self.expect(Kind::Percent)?;
        let span = self.end_span(span);
        let selector = &self.source_text[(span.start as usize)..(span.end as usize)];

        Ok(Some(Atom::from(selector)))
    }

    fn parse_combinator(&mut self, space: bool) -> Option<Combinator> {
        let mut span = self.start_span();
        let kind = self.parse_combinator_kind(space)?;

        span.end = if kind != CombinatorKind::Descendant {
            self.eat(self.cur_kind());
            self.prev_token_end
        } else {
            self.cur_token().start
        };

        Some(self.ast.combinator(span, kind))
    }

    // doesn't eat the token
    fn parse_combinator_kind(&self, space: bool) -> Option<CombinatorKind> {
        Some(match self.cur_kind() {
            Kind::Plus => CombinatorKind::NextSibling,
            Kind::Tilde => CombinatorKind::LaterSibling,
            Kind::RAngle => CombinatorKind::Child,
            Kind::Pipe2 => CombinatorKind::Column,
            _ => {
                if space && self.prev_token_end != self.cur_token().start {
                    return Some(CombinatorKind::Descendant);
                }
                return None;
            }
        })
    }
}
