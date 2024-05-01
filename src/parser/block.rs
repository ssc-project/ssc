use super::{
    errors::parse::{InvalidElseif, UnexpectedEof},
    Parser,
};
use crate::ast::template::{
    AwaitBlock, Block, EachBlock, EachBlockMetadata, Fragment,
    FragmentNodeKind, IfBlock, KeyBlock,
};
use lazy_static::lazy_static;
use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::IdentifierName;
use oxc_span::{Atom, GetSpan, SourceType, Span, SPAN};
use regex::Regex;
use rustc_hash::FxHashMap;

lazy_static! {
    static ref REGEX_AS_OR_CLOSE_CURLY_BRACE: Regex =
        Regex::new("as|}").unwrap();
}

impl<'a> Parser<'a> {
    pub fn parse_block(&mut self) -> Option<Block<'a>> {
        let start = self.index;
        if !self.eat("{", false) {
            return None;
        }

        self.allow_whitespace();

        if self.eat("#", false) {
            if self.eat("if", false) {
                self.parse_if_block(start, false).map(Block::If)
            } else if self.eat("each", false) {
                self.parse_each_block(start).map(Block::Each)
            } else if self.eat("await", false) {
                self.parse_await_block(start).map(Block::Await)
            } else if self.eat("key", false) {
                self.parse_key_block(start).map(Block::Key)
            } else {
                todo!()
            }
        } else if (!self.state.is_inside_if_block
            && !self.state.is_inside_each_block)
            && (self.eat("/", false) || self.eat(":", false))
        {
            // TODO: report error
            None
        } else {
            self.index = start;
            None
        }
    }

    fn parse_if_block(
        &mut self,
        start: usize,
        elseif: bool,
    ) -> Option<IfBlock<'a>> {
        let state_changed = if self.state.is_inside_if_block {
            false
        } else {
            self.state.is_inside_if_block = true;
            true
        };
        self.require_whitespace();
        let test = self.parse_expression();

        self.allow_whitespace();
        self.eat("}", true);

        let Some(test) = test else {
            // TODO: report error
            return None;
        };

        let consequent = self.parse_fragment(false);

        self.allow_whitespace();
        let block_start = self.index;
        self.eat("{", true);

        let alternate = if self.eat("/", false) {
            self.eat("if", true);
            self.allow_whitespace();
            self.eat("}", true);
            None
        } else if self.eat(":", false) {
            self.eat("else", true);
            let elseif = if self.eat("if", false) {
                self.error(InvalidElseif(Span::new(
                    start as u32,
                    self.index as u32,
                )));
                true
            } else {
                self.allow_whitespace();
                self.eat("if", false)
            };

            if elseif {
                let mut nodes = OxcVec::new_in(self.allocator);
                if let Some(block) = self.parse_if_block(block_start, true) {
                    nodes.push(FragmentNodeKind::Block(Block::If(block)));
                }
                Some(Fragment { transparent: false, nodes })
            } else {
                self.allow_whitespace();
                self.eat("}", true);
                let alternate = self.parse_fragment(false);
                self.allow_whitespace();
                self.eat("{/if", true);
                self.allow_whitespace();
                self.eat("}", true);
                Some(alternate)
            }
        } else {
            None
        };

        if state_changed {
            self.state.is_inside_if_block = false;
        }

        Some(IfBlock {
            span: Span::new(start as u32, self.index as u32),
            elseif,
            test,
            consequent,
            alternate,
        })
    }

    fn parse_each_block(&mut self, start: usize) -> Option<EachBlock<'a>> {
        self.require_whitespace();
        let expression_start = self.index;
        self.read_until(&REGEX_AS_OR_CLOSE_CURLY_BRACE);
        let expression_end = self.index;
        if self.eat("}", false) {
            // TODO: report error
            return None;
        }

        let expression = if self.eat("as", false) {
            let parser = crate::oxc_parser::Parser::new(
                self.allocator,
                &self.source_text[..expression_end],
                SourceType::default(),
            );
            match parser.parse_expression_at(expression_start) {
                Ok(expression) => expression,
                Err(error) => {
                    // TODO: modify the error
                    self.error(error);
                    return None;
                }
            }
        } else {
            // already reported unexpected EOF
            return None;
        };
        self.require_whitespace();
        let parser = crate::oxc_parser::Parser::new(
            self.allocator,
            self.source_text,
            SourceType::default(),
        );
        let context = match parser.parse_binding_pattern_at(self.index) {
            Ok(context) => {
                self.index = context.span().end as usize;
                context
            }
            Err(error) => {
                self.error(error);
                return None;
            }
        };
        self.allow_whitespace();

        let index = if self.eat(",", false) {
            self.allow_whitespace();
            match self.parse_identifier() {
                Ok(index) => {
                    self.allow_whitespace();
                    Some(index)
                }
                Err(error) => {
                    self.error(error);
                    None
                }
            }
        } else {
            None
        };

        let key = if self.eat("(", false) {
            self.allow_whitespace();
            let key = self.parse_expression();
            self.allow_whitespace();
            self.eat(")", true);
            key
        } else {
            None
        };

        self.allow_whitespace();
        self.eat("}", true);

        let mut nodes = OxcVec::new_in(self.allocator);

        let has_fallback = 'body: {
            while self.index < self.source_text.len() {
                self.allow_whitespace();
                if self.eat("{", false) {
                    if self.eat(":", false) {
                        self.eat("else", true);
                        self.allow_whitespace();
                        self.eat("}", true);
                        break 'body true;
                    } else {
                        self.eat("/each", true);
                        self.allow_whitespace();
                        self.eat("}", false);
                        break 'body false;
                    }
                }

                if let Some(comment) = self.parse_comment() {
                    nodes.push(FragmentNodeKind::Comment(comment));
                    continue;
                }

                if let Some(element_like) = self.parse_element_like(false) {
                    nodes.push(FragmentNodeKind::ElementLike(element_like));
                    continue;
                }

                if let Some(block) = self.parse_block() {
                    nodes.push(FragmentNodeKind::Block(block));
                    continue;
                }

                if let Some(tag) = self.parse_tag() {
                    nodes.push(FragmentNodeKind::Tag(tag));
                    continue;
                }

                if let Some(text) = self.parse_text() {
                    nodes.push(FragmentNodeKind::Text(text));
                }
            }

            if self.index >= self.source_text.len() {
                self.error(UnexpectedEof(Span::new(
                    self.index as u32,
                    self.index as u32,
                )));
                return None;
            };

            false
        };

        let body = Fragment { nodes, transparent: true };

        let fallback = if !has_fallback {
            None
        } else {
            let mut nodes = OxcVec::new_in(self.allocator);

            'fallback: {
                while self.index < self.source_text.len() {
                    self.allow_whitespace();
                    if self.eat("{", false) {
                        self.eat("/each", true);
                        self.allow_whitespace();
                        self.eat("}", true);
                        break 'fallback;
                    }

                    if let Some(comment) = self.parse_comment() {
                        nodes.push(FragmentNodeKind::Comment(comment));
                        continue;
                    }

                    if let Some(element_like) = self.parse_element_like(false) {
                        nodes.push(FragmentNodeKind::ElementLike(element_like));
                        continue;
                    }

                    if let Some(block) = self.parse_block() {
                        nodes.push(FragmentNodeKind::Block(block));
                        continue;
                    }

                    if let Some(tag) = self.parse_tag() {
                        nodes.push(FragmentNodeKind::Tag(tag));
                        continue;
                    }

                    if let Some(text) = self.parse_text() {
                        nodes.push(FragmentNodeKind::Text(text));
                    }
                }

                if self.index >= self.source_text.len() {
                    self.error(UnexpectedEof(Span::new(
                        self.index as u32,
                        self.index as u32,
                    )));
                    return None;
                };
            }

            Some(Fragment { nodes, transparent: false })
        };

        Some(EachBlock {
            span: Span::new(start as u32, self.index as u32),
            expression,
            context,
            body,
            fallback,
            index: index.clone(),
            key,
            // TODO: figure this out
            metadata: EachBlockMetadata {
                contains_group_binding: false,
                array_name: None,
                index: IdentifierName::new(SPAN, Atom::from("")),
                item: IdentifierName::new(SPAN, Atom::from("")),
                declarations: FxHashMap::default(),
                references: OxcVec::new_in(self.allocator),
                is_controlled: false,
            },
        })
    }

    fn parse_await_block(&mut self, start: usize) -> Option<AwaitBlock<'a>> {
        self.require_whitespace();
        let Some(expression) = self.parse_expression() else {
            return None;
        };
        self.allow_whitespace();
        let (value, error, pending, then, catch) = if self.eat("then", false) {
            self.require_whitespace();
            let parser = crate::oxc_parser::Parser::new(
                self.allocator,
                self.source_text,
                SourceType::default(),
            );
            let Ok(value) = parser.parse_binding_pattern_at(self.index) else {
                // TODO: report error
                return None;
            };

            self.index = value.span().end as usize;
            self.allow_whitespace();
            self.eat("}", true);

            let mut nodes = OxcVec::new_in(self.allocator);

            'then: {
                while self.index < self.source_text.len() {
                    self.allow_whitespace();
                    if self.eat("{", false) {
                        self.eat("/await", true);
                        self.allow_whitespace();
                        self.eat("}", true);
                        break 'then;
                    }

                    if let Some(comment) = self.parse_comment() {
                        nodes.push(FragmentNodeKind::Comment(comment));
                        continue;
                    }

                    if let Some(element_like) = self.parse_element_like(false) {
                        nodes.push(FragmentNodeKind::ElementLike(element_like));
                        continue;
                    }

                    if let Some(block) = self.parse_block() {
                        nodes.push(FragmentNodeKind::Block(block));
                        continue;
                    }

                    if let Some(tag) = self.parse_tag() {
                        nodes.push(FragmentNodeKind::Tag(tag));
                        continue;
                    }

                    if let Some(text) = self.parse_text() {
                        nodes.push(FragmentNodeKind::Text(text));
                    }
                }

                if self.index >= self.source_text.len() {
                    self.error(UnexpectedEof(Span::new(
                        self.index as u32,
                        self.index as u32,
                    )));
                    return None;
                };
            }

            let then = Fragment { nodes, transparent: false };

            (Some(value), None, None, Some(then), None)
        } else if self.eat("catch", false) {
            self.require_whitespace();
            let parser = crate::oxc_parser::Parser::new(
                self.allocator,
                self.source_text,
                SourceType::default(),
            );
            let Ok(error) = parser.parse_binding_pattern_at(self.index) else {
                // TODO: report error
                return None;
            };

            self.index = error.span().end as usize;
            self.allow_whitespace();
            self.eat("}", true);

            let mut nodes = OxcVec::new_in(self.allocator);

            'catch: {
                while self.index < self.source_text.len() {
                    self.allow_whitespace();
                    if self.eat("{", false) {
                        self.eat("/await", true);
                        self.allow_whitespace();
                        self.eat("}", true);
                        break 'catch;
                    }

                    if let Some(comment) = self.parse_comment() {
                        nodes.push(FragmentNodeKind::Comment(comment));
                        continue;
                    }

                    if let Some(element_like) = self.parse_element_like(false) {
                        nodes.push(FragmentNodeKind::ElementLike(element_like));
                        continue;
                    }

                    if let Some(block) = self.parse_block() {
                        nodes.push(FragmentNodeKind::Block(block));
                        continue;
                    }

                    if let Some(tag) = self.parse_tag() {
                        nodes.push(FragmentNodeKind::Tag(tag));
                        continue;
                    }

                    if let Some(text) = self.parse_text() {
                        nodes.push(FragmentNodeKind::Text(text));
                    }
                }

                if self.index >= self.source_text.len() {
                    self.error(UnexpectedEof(Span::new(
                        self.index as u32,
                        self.index as u32,
                    )));
                    return None;
                };
            }

            let catch = Fragment { nodes, transparent: false };

            (None, Some(error), None, None, Some(catch))
        } else {
            self.eat("}", true);

            let mut value = None;
            let mut error = None;
            let mut nodes = OxcVec::new_in(self.allocator);

            'pending: {
                while self.index < self.source_text.len() {
                    self.allow_whitespace();
                    if self.eat("{", false) {
                        if self.eat(":", false) {
                            if self.eat("then", false) {
                                self.require_whitespace();
                                let parser = crate::oxc_parser::Parser::new(
                                    self.allocator,
                                    self.source_text,
                                    SourceType::default(),
                                );
                                let Ok(pattern) =
                                    parser.parse_binding_pattern_at(self.index)
                                else {
                                    // TODO: report error
                                    return None;
                                };
                                self.index = pattern.span().end as usize;
                                value = Some(pattern);
                            } else if self.eat("catch", false) {
                                self.require_whitespace();
                                let parser = crate::oxc_parser::Parser::new(
                                    self.allocator,
                                    self.source_text,
                                    SourceType::default(),
                                );
                                let Ok(pattern) =
                                    parser.parse_binding_pattern_at(self.index)
                                else {
                                    // TODO: report error
                                    return None;
                                };
                                self.index = pattern.span().end as usize;
                                error = Some(pattern);
                            } else {
                                // TODO: report error
                            }
                        } else {
                            self.eat("/await", true);
                        }
                        self.allow_whitespace();
                        self.eat("}", true);
                        break 'pending;
                    }

                    if let Some(comment) = self.parse_comment() {
                        nodes.push(FragmentNodeKind::Comment(comment));
                        continue;
                    }

                    if let Some(element_like) = self.parse_element_like(false) {
                        nodes.push(FragmentNodeKind::ElementLike(element_like));
                        continue;
                    }

                    if let Some(block) = self.parse_block() {
                        nodes.push(FragmentNodeKind::Block(block));
                        continue;
                    }

                    if let Some(tag) = self.parse_tag() {
                        nodes.push(FragmentNodeKind::Tag(tag));
                        continue;
                    }

                    if let Some(text) = self.parse_text() {
                        nodes.push(FragmentNodeKind::Text(text));
                    }
                }

                if self.index >= self.source_text.len() {
                    self.error(UnexpectedEof(Span::new(
                        self.index as u32,
                        self.index as u32,
                    )));
                    return None;
                };
            }

            let pending = Fragment { nodes, transparent: false };

            let then = if value.is_some() {
                let mut nodes = OxcVec::new_in(self.allocator);

                'then: {
                    while self.index < self.source_text.len() {
                        self.allow_whitespace();
                        if self.eat("{", false) {
                            if self.eat(":", false) {
                                self.eat("catch", true);
                                self.require_whitespace();
                                let parser = crate::oxc_parser::Parser::new(
                                    self.allocator,
                                    self.source_text,
                                    SourceType::default(),
                                );
                                let Ok(pattern) =
                                    parser.parse_binding_pattern_at(self.index)
                                else {
                                    // TODO: report error
                                    return None;
                                };
                                self.index = pattern.span().end as usize;
                                error = Some(pattern);
                            } else {
                                self.eat("/await", true);
                            }
                            self.allow_whitespace();
                            self.eat("}", true);
                            break 'then;
                        }

                        if let Some(comment) = self.parse_comment() {
                            nodes.push(FragmentNodeKind::Comment(comment));
                            continue;
                        }

                        if let Some(element_like) =
                            self.parse_element_like(false)
                        {
                            nodes.push(FragmentNodeKind::ElementLike(
                                element_like,
                            ));
                            continue;
                        }

                        if let Some(block) = self.parse_block() {
                            nodes.push(FragmentNodeKind::Block(block));
                            continue;
                        }

                        if let Some(tag) = self.parse_tag() {
                            nodes.push(FragmentNodeKind::Tag(tag));
                            continue;
                        }

                        if let Some(text) = self.parse_text() {
                            nodes.push(FragmentNodeKind::Text(text));
                        }
                    }

                    if self.index >= self.source_text.len() {
                        self.error(UnexpectedEof(Span::new(
                            self.index as u32,
                            self.index as u32,
                        )));
                        return None;
                    };
                }

                Some(Fragment { nodes, transparent: false })
            } else {
                None
            };

            let catch = if error.is_some() {
                let mut nodes = OxcVec::new_in(self.allocator);

                'catch: {
                    while self.index < self.source_text.len() {
                        self.allow_whitespace();
                        if self.eat("{", false) {
                            self.eat("/await", true);
                            self.allow_whitespace();
                            self.eat("}", true);
                            break 'catch;
                        }

                        if let Some(comment) = self.parse_comment() {
                            nodes.push(FragmentNodeKind::Comment(comment));
                            continue;
                        }

                        if let Some(element_like) =
                            self.parse_element_like(false)
                        {
                            nodes.push(FragmentNodeKind::ElementLike(
                                element_like,
                            ));
                            continue;
                        }

                        if let Some(block) = self.parse_block() {
                            nodes.push(FragmentNodeKind::Block(block));
                            continue;
                        }

                        if let Some(tag) = self.parse_tag() {
                            nodes.push(FragmentNodeKind::Tag(tag));
                            continue;
                        }

                        if let Some(text) = self.parse_text() {
                            nodes.push(FragmentNodeKind::Text(text));
                        }
                    }

                    if self.index >= self.source_text.len() {
                        self.error(UnexpectedEof(Span::new(
                            self.index as u32,
                            self.index as u32,
                        )));
                        return None;
                    };
                }

                Some(Fragment { nodes, transparent: false })
            } else {
                None
            };

            (value, error, Some(pending), then, catch)
        };
        Some(AwaitBlock {
            span: Span::new(start as u32, self.index as u32),
            expression,
            value,
            error,
            pending,
            then,
            catch,
        })
    }

    fn parse_key_block(&mut self, start: usize) -> Option<KeyBlock<'a>> {
        self.require_whitespace();
        let Some(expression) = self.parse_expression() else {
            return None;
        };
        let mut nodes = OxcVec::new_in(self.allocator);

        'body: {
            while self.index < self.source_text.len() {
                self.allow_whitespace();
                if self.eat("{", false) {
                    self.eat("/key", true);
                    self.allow_whitespace();
                    self.eat("}", true);
                    break 'body;
                }

                if let Some(comment) = self.parse_comment() {
                    nodes.push(FragmentNodeKind::Comment(comment));
                    continue;
                }

                if let Some(element_like) = self.parse_element_like(false) {
                    nodes.push(FragmentNodeKind::ElementLike(element_like));
                    continue;
                }

                if let Some(block) = self.parse_block() {
                    nodes.push(FragmentNodeKind::Block(block));
                    continue;
                }

                if let Some(tag) = self.parse_tag() {
                    nodes.push(FragmentNodeKind::Tag(tag));
                    continue;
                }

                if let Some(text) = self.parse_text() {
                    nodes.push(FragmentNodeKind::Text(text));
                }
            }

            if self.index >= self.source_text.len() {
                self.error(UnexpectedEof(Span::new(
                    self.index as u32,
                    self.index as u32,
                )));
                return None;
            };
        }

        let fragment = Fragment { nodes, transparent: false };

        Some(KeyBlock {
            span: Span::new(start as u32, self.index as u32),
            expression,
            fragment,
        })
    }
}
