use std::collections::HashSet;

use lazy_static::lazy_static;
use oxc_allocator::{Box as OxcBox, Vec as OxcVec};
use oxc_ast::ast::{
    Expression, IdentifierName, IdentifierReference, StringLiteral,
};
use oxc_span::{Atom, GetSpan, Span};
use phf::phf_set;
use regex::Regex;

use super::{
    errors::{
        attributes::DuplicateAttribute,
        elements::{InvalidElementContent, InvalidTagName},
        parse::{
            InvalidBlockPlacement, InvalidTagPlacement, MissingAttributeValue,
        },
        special_elements::{
            InvalidSelfPlacement, InvalidSvelteElementPlacement,
            InvalidSvelteTag,
        },
    },
    Parser,
};
use crate::{
    ast::template::{
        AnimateDirective, Attribute, AttributeSequenceValue, AttributeValue,
        BindDirective, BindDirectiveExpression, ClassDirective,
        ClassDirectiveMetadata, Component, Directive, ElementAttribute,
        ElementLike, ExpressionTag, ExpressionTagMetadata, Fragment,
        FragmentNodeKind, LetDirective, LetDirectiveExpression, OnDirective,
        RegularElement, RegularElementMetadata, SlotElement, SlotElementName,
        SpreadAttribute, SpreadAttributeMetadata, StyleDirective,
        StyleDirectiveMetadata, StyleDirectiveModifier, SvelteBody,
        SvelteBodyName, SvelteComponent, SvelteComponentName, SvelteDocument,
        SvelteDocumentName, SvelteElement, SvelteElementMetadata,
        SvelteElementName, SvelteFragment, SvelteFragmentName, SvelteHead,
        SvelteHeadName, SvelteOptionsRaw, SvelteOptionsRawName, SvelteSelf,
        SvelteSelfName, SvelteWindow, SvelteWindowName, Text, TitleElement,
        TitleElementName, TransitionDirective, TransitionDirectiveModifier,
        UseDirective,
    },
    parser::{
        errors::{
            attributes::EmptyAttributeShorthand,
            parse::{
                EmptyDirectiveName, InvalidClosingTag, InvalidDirectiveValue,
                UnexpectedEof, UnexpectedToken,
            },
            special_elements::{
                DuplicateSvelteElement, InvalidSvelteComponentDefinition,
                InvalidSvelteElementDefinition,
                MissingSvelteComponentDefinition,
                MissingSvelteElementDefinition,
            },
        },
        utils::html::decode_character_references,
    },
};

const ROOT_ONLY_META_TAGS: phf::Set<&'static str> = phf_set![
    "svelte:head",
    "svelte:options",
    "svelte:window",
    "svelte:document",
    "svelte:body",
];

const META_TAGS: phf::Set<&'static str> = phf_set![
    "svelte:head",
    "svelte:options",
    "svelte:window",
    "svelte:document",
    "svelte:body",
    "svelte:element",
    "svelte:component",
    "svelte:self",
    "svelte:fragment",
];

lazy_static! {
    static ref REGEX_SELF: Regex =
        Regex::new(r"^svelte:self(?:\s|\/|>)").unwrap();
    static ref REGEX_COMPONENT: Regex =
        Regex::new(r"^svelte:component(?:\s|\/|>)").unwrap();
    static ref REGEX_ELEMENT: Regex =
        Regex::new(r"^svelte:element(?:\s|\/|>)").unwrap();
    static ref REGEX_SLOT: Regex =
        Regex::new(r"^svelte:fragment(?:\s|\/|>)").unwrap();
    static ref REGEX_WHITESPACE_OR_SLASH_OR_CLOSING_TAG: Regex =
        Regex::new(r"(\s|\/|>)").unwrap();
    static ref REGEX_VALID_TAG_NAME: Regex =
        Regex::new(r"^\!?[a-zA-Z]{1,}:?[a-zA-Z0-9\-]*").unwrap();
    static ref REGEX_TOKEN_ENDING_CHARACTER: Regex =
        Regex::new(r#"[\s=\/>"']"#).unwrap();
    static ref REGEX_ATTRIBUTE_VALUE: Regex =
        Regex::new(r#"(?:"[^"]*"|'[^']*'|[^">\s]+)"#).unwrap();
    static ref REGEX_STARTS_WITH_QUOTE_CHARACTER: Regex =
        Regex::new(r#"^["']"#).unwrap();
    static ref REGEX_CAPITAL_LETTER: Regex = Regex::new("[A-Z]").unwrap();
    static ref REGEX_STARTS_WITH_INVALID_ATTR_VALUE: Regex =
        Regex::new(r#"^(\/>|[\s"'=<>`])"#).unwrap();
    static ref REGEX_NOT_LOWERCASE: Regex = Regex::new("[^a-z]").unwrap();
}

impl<'a> Parser<'a> {
    pub fn parse_element_like(
        &mut self,
        is_root: bool,
    ) -> Option<ElementLike<'a>> {
        let start = self.index;
        if !self.eat("<", false) {
            return None;
        }

        let is_closing_tag = self.eat("/", false);

        let name = self.allocator.alloc(self.parse_tag_name().to_string());

        let mut unique_names = HashSet::new();
        let mut attributes = OxcVec::new_in(self.allocator);

        self.allow_whitespace();

        if is_closing_tag {
            self.eat(">", true);
            self.error(InvalidClosingTag(
                Span::new(start as u32, self.index as u32),
                name.to_string(),
            ));
            return None;
        }

        while let Some(attribute) = self.parse_attribute() {
            let attribute_name = match &attribute {
                ElementAttribute::Attribute(attribute) => {
                    Some(attribute.name.as_str().to_string())
                }
                ElementAttribute::Directive(directive) => {
                    if let Directive::Bind(bind) = directive {
                        Some(bind.name.as_str().to_string())
                    } else {
                        None
                    }
                }
                _ => None,
            };

            if let Some(attribute_name) = attribute_name {
                if unique_names.contains(&attribute_name) {
                    self.error(DuplicateAttribute(attribute.span()));
                } else if attribute_name != "this" {
                    unique_names.insert(attribute_name);
                }
            }

            attributes.push(attribute);
            self.allow_whitespace();
        }

        let is_self_closing_tag = self.eat("/", false);

        self.eat(">", true);

        let mut nodes = OxcVec::new_in(self.allocator);

        if !is_self_closing_tag {
            if name == "svelte:head" {
                self.state.inside_head = true;
            }
            'content: {
                while self.index < self.source_text.len() {
                    let end_tag_start = self.index;
                    if self.eat("</", false) {
                        let end_tag_name = self.parse_tag_name();
                        if name == end_tag_name {
                            self.allow_whitespace();
                            self.eat(">", true);
                        } else {
                            self.index = end_tag_start;
                        }
                        break 'content;
                    }

                    self.allow_whitespace();

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
            if name == "svelte:head" {
                self.state.inside_head = false;
            }
        };

        let fragment = Fragment { nodes, transparent: true };

        let span = Span::new(start as u32, self.index as u32);

        if ROOT_ONLY_META_TAGS.contains(name) {
            if !is_self_closing_tag && name != "svelte:head" {
                self.error(InvalidElementContent(span, name.to_string()));
            }
            if !is_root {
                self.error(InvalidSvelteElementPlacement(
                    span,
                    name.to_string(),
                ));
            }
            if self.meta_tags.contains(name) {
                self.error(DuplicateSvelteElement(span, name.to_string()));
            } else {
                self.meta_tags.insert(name.to_string());
            }
        }

        Some(if name == "svelte:head" {
            ElementLike::SvelteHead(SvelteHead {
                span,
                name: SvelteHeadName,
                attributes,
                fragment,
            })
        } else if name == "svelte:options" {
            ElementLike::SvelteOptionsRaw(SvelteOptionsRaw {
                span,
                name: SvelteOptionsRawName,
                attributes,
                fragment,
            })
        } else if name == "svelte:window" {
            ElementLike::SvelteWindow(SvelteWindow {
                span,
                name: SvelteWindowName,
                attributes,
                fragment,
            })
        } else if name == "svelte:document" {
            ElementLike::SvelteDocument(SvelteDocument {
                span,
                name: SvelteDocumentName,
                attributes,
                fragment,
            })
        } else if name == "svelte:body" {
            ElementLike::SvelteBody(SvelteBody {
                span,
                name: SvelteBodyName,
                attributes,
                fragment,
            })
        } else if name == "svelte:element" {
            let attr_idx = attributes.iter().position(|attribute| {
                if let ElementAttribute::Attribute(Attribute { name, .. }) =
                    attribute
                {
                    if name.as_str() == "this" {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            });
            let Some(attr_idx) = attr_idx else {
                self.error(MissingSvelteElementDefinition(span));
                return None;
            };
            let mut this_attribute = attributes.remove(attr_idx);

            let expression = if let ElementAttribute::Attribute(ref mut attr) =
                this_attribute
            {
                if let AttributeValue::Sequence(ref mut attr_values) =
                    attr.value
                {
                    if attr_values.len() == 1 {
                        Some(match attr_values.remove(0) {
                            AttributeSequenceValue::ExpressionTag(tag) => {
                                tag.expression
                            }
                            AttributeSequenceValue::Text(text) => {
                                Expression::StringLiteral(OxcBox::new_in(
                                    StringLiteral {
                                        span: text.span,
                                        value: text.data,
                                    },
                                    self.allocator,
                                ))
                            }
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            let Some(expression) = expression else {
                self.error(InvalidSvelteElementDefinition(span));
                return None;
            };

            ElementLike::SvelteElement(SvelteElement {
                span,
                name: SvelteElementName,
                attributes,
                fragment,
                expression,
                metadata: SvelteElementMetadata { svg: false, scoped: false },
            })
        } else if name == "svelte:component" {
            let attr_idx = attributes.iter().position(|attribute| {
                if let ElementAttribute::Attribute(Attribute { name, .. }) =
                    attribute
                {
                    if name.as_str() == "this" {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            });
            let Some(attr_idx) = attr_idx else {
                self.error(MissingSvelteComponentDefinition(span));
                return None;
            };
            let mut this_attribute = attributes.remove(attr_idx);

            let expression = if let ElementAttribute::Attribute(ref mut attr) =
                this_attribute
            {
                if let AttributeValue::Sequence(ref mut attr_values) =
                    attr.value
                {
                    if attr_values.len() == 1 {
                        if let AttributeSequenceValue::ExpressionTag(tag) =
                            attr_values.remove(0)
                        {
                            Some(tag.expression)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            let Some(expression) = expression else {
                self.error(InvalidSvelteComponentDefinition(span));
                return None;
            };

            ElementLike::SvelteComponent(SvelteComponent {
                span,
                name: SvelteComponentName,
                attributes,
                fragment,
                expression,
            })
        } else if name == "svelte:self" {
            ElementLike::SvelteSelf(SvelteSelf {
                span,
                name: SvelteSelfName,
                attributes,
                fragment,
            })
        } else if name == "svelte:fragment" {
            ElementLike::SvelteFragment(SvelteFragment {
                span,
                name: SvelteFragmentName,
                attributes,
                fragment,
            })
        } else if REGEX_CAPITAL_LETTER
            .is_match(&char::from(name.as_bytes()[0]).to_string())
        {
            ElementLike::Component(Component {
                span,
                name: Atom::from(name as &str),
                attributes,
                fragment,
            })
        } else if name == "title" && self.state.inside_head {
            ElementLike::Title(TitleElement {
                span,
                name: TitleElementName,
                attributes,
                fragment,
            })
        } else if name == "slot" {
            ElementLike::Slot(SlotElement {
                span,
                name: SlotElementName,
                attributes,
                fragment,
            })
        } else {
            ElementLike::Regular(RegularElement {
                span,
                name: Atom::from(name as &str),
                attributes,
                fragment,
                metadata: RegularElementMetadata {
                    svg: false,
                    has_spread: false,
                    scoped: false,
                },
            })
        })
    }

    pub(crate) fn parse_fragment(&mut self, transparent: bool) -> Fragment<'a> {
        let mut nodes = OxcVec::new_in(self.allocator);
        let mut empty_round = false;

        while self.index < self.source_text.len() {
            self.allow_whitespace();

            if let Some(comment) = self.parse_comment() {
                nodes.push(FragmentNodeKind::Comment(comment));
                empty_round = false;
                continue;
            }

            if let Some(element_like) = self.parse_element_like(false) {
                nodes.push(FragmentNodeKind::ElementLike(element_like));
                empty_round = false;
                continue;
            }

            if let Some(block) = self.parse_block() {
                nodes.push(FragmentNodeKind::Block(block));
                empty_round = false;
                continue;
            }

            if let Some(tag) = self.parse_tag() {
                nodes.push(FragmentNodeKind::Tag(tag));
                empty_round = false;
                continue;
            }

            if let Some(text) = self.parse_text() {
                nodes.push(FragmentNodeKind::Text(text));
                empty_round = false;
                continue;
            }

            if empty_round {
                break;
            } else {
                empty_round = true;
            }
        }

        Fragment { nodes, transparent }
    }

    pub fn parse_tag_name(&mut self) -> &str {
        let start = self.index;

        if self.match_regex(&REGEX_SELF).is_some() {
            self.index += 11;
            // TODO: report error if the placement of this tag isn't right
            if !self.state.is_inside_if_block
                && !self.state.is_inside_each_block
                && !self.state.is_inside_snippet_block
            {
                self.error(InvalidSelfPlacement(Span::new(
                    start as u32,
                    self.index as u32,
                )));
            }
            return "svelte:self";
        }

        if self.match_regex(&REGEX_COMPONENT).is_some() {
            self.index += 16;
            return "svelte:component";
        }

        if self.match_regex(&REGEX_ELEMENT).is_some() {
            self.index += 14;
            return "svelte:element";
        }

        if self.match_regex(&REGEX_SLOT).is_some() {
            self.index += 15;
            return "svelte:fragment";
        }

        let name = self
            .read_until(&REGEX_WHITESPACE_OR_SLASH_OR_CLOSING_TAG)
            .to_string();
        let name = self.allocator.alloc(name);

        if META_TAGS.contains(name) {
            return name;
        }

        let name_span = Span::new(start as u32, (start + name.len()) as u32);

        if name.starts_with("svelte:") {
            self.error(InvalidSvelteTag(
                name_span,
                META_TAGS.iter().copied().collect::<Vec<&str>>().join(", "),
            ));
            return name;
        }

        if !REGEX_VALID_TAG_NAME.is_match(name) {
            self.error(InvalidTagName(name_span));
        }

        name
    }

    fn parse_attribute(&mut self) -> Option<ElementAttribute<'a>> {
        let start = self.index;

        if self.eat("{", false) {
            self.allow_whitespace();

            if self.eat("...", false) {
                let expression = self.parse_expression();

                self.allow_whitespace();
                self.eat("}", true);

                let Some(expression) = expression else {
                    return None;
                };

                return Some(ElementAttribute::SpreadAttribute(
                    SpreadAttribute {
                        span: Span::new(start as u32, self.index as u32),
                        expression,
                        metadata: SpreadAttributeMetadata {
                            contains_call_expression: false,
                            dynamic: false,
                        },
                    },
                ));
            } else {
                let value_start = self.index;
                let Some(name) = self.parse_identifier(false) else {
                    self.error(EmptyAttributeShorthand(Span::new(
                        self.index as u32,
                        self.index as u32,
                    )));
                    return None;
                };

                self.allow_whitespace();
                self.eat("}", true);

                let mut value = OxcVec::new_in(self.allocator);

                let span = Span::new(
                    value_start as u32,
                    (value_start + name.name.len()) as u32,
                );

                value.push(AttributeSequenceValue::ExpressionTag(
                    ExpressionTag {
                        span,
                        expression: Expression::Identifier(OxcBox::new_in(
                            IdentifierReference::new(span, name.name.clone()),
                            self.allocator,
                        )),
                        metadata: ExpressionTagMetadata {
                            contains_call_expression: false,
                            dynamic: false,
                        },
                    },
                ));

                return Some(ElementAttribute::Attribute(Attribute {
                    span: Span::new(start as u32, self.index as u32),
                    name: Atom::from(name.name),
                    value: AttributeValue::Sequence(value),
                }));
            }
        }

        let name = self
            .allocator
            .alloc(self.read_until(&REGEX_TOKEN_ENDING_CHARACTER).to_string());

        if name.is_empty() {
            return None;
        }

        let mut end = self.index;

        self.allow_whitespace();

        let colon_idx = name.chars().position(|ch| ch == ':');

        let value = if self.eat("=", false) {
            self.allow_whitespace();
            let value = self.parse_attribute_value();
            end = self.index;
            AttributeValue::Sequence(value)
        } else {
            AttributeValue::Bool(true)
        };

        if let Some(colon_idx) = colon_idx {
            let directive_type = &name[0..colon_idx];
            let mut directive_parts = name[(colon_idx + 1)..].split('|');
            let directive_name =
                if let Some(directive_name) = directive_parts.next() {
                    directive_name.to_string()
                } else {
                    self.error(EmptyDirectiveName(
                        Span::new(
                            (start + colon_idx + 1) as u32,
                            (start + colon_idx + 1) as u32,
                        ),
                        directive_type.to_string(),
                    ));
                    return None;
                };
            let name = Atom::from(
                self.allocator.alloc(directive_name.clone()) as &str
            );

            let raw_modifiers: Vec<&str> = directive_parts.collect();
            let span = Span::new(start as u32, end as u32);

            if directive_type == "style" {
                let mut modifiers = OxcVec::new_in(self.allocator);

                if raw_modifiers.len() == 1 {
                    if raw_modifiers[0] == "important" {
                        modifiers.push(StyleDirectiveModifier::Important);
                    } else {
                        // TODO: report error
                    }
                } else if !modifiers.is_empty() {
                    // TODO: report error
                }

                return Some(ElementAttribute::Directive(Directive::Style(
                    StyleDirective {
                        span,
                        name,
                        value,
                        modifiers,
                        metadata: StyleDirectiveMetadata { dynamic: false },
                    },
                )));
            }

            let first_value = match value {
                AttributeValue::Bool(_) => None,
                AttributeValue::Sequence(mut seq) => {
                    if !seq.is_empty() {
                        Some((seq.remove(0), !seq.is_empty()))
                    } else {
                        None
                    }
                }
            };

            let expression =
                if let Some((first_value, has_more_than_one_value)) =
                    first_value
                {
                    match first_value {
                        AttributeSequenceValue::ExpressionTag(tag) => {
                            Some(tag.expression)
                        }
                        AttributeSequenceValue::Text(text) => {
                            if has_more_than_one_value {
                                self.error(InvalidDirectiveValue(text.span));
                            }
                            None
                        }
                    }
                } else {
                    None
                };

            return Some(ElementAttribute::Directive(
                if directive_name == "animate" {
                    Directive::Animate(AnimateDirective {
                        span,
                        name,
                        expression,
                    })
                } else if directive_name == "bind" {
                    Directive::Bind(BindDirective {
                        span,
                        name: name.clone(),
                        expression: if let Some(expression) = expression {
                            if let Expression::MemberExpression(
                                member_expression,
                            ) = expression
                            {
                                BindDirectiveExpression::MemberExpression(
                                    member_expression.unbox(),
                                )
                            } else {
                                self.error(InvalidDirectiveValue(span));
                                return None;
                            }
                        } else {
                            BindDirectiveExpression::Identifier(
                                IdentifierName::new(
                                    Span::new(
                                        (start + colon_idx + 1) as u32,
                                        span.end,
                                    ),
                                    name,
                                ),
                            )
                        },
                    })
                } else if directive_name == "class" {
                    Directive::Class(ClassDirective {
                        span,
                        name: name.clone(),
                        expression: if let Some(expression) = expression {
                            expression
                        } else {
                            Expression::Identifier(OxcBox::new_in(
                                IdentifierReference::new(
                                    Span::new(
                                        (start + colon_idx + 1) as u32,
                                        span.end,
                                    ),
                                    name,
                                ),
                                self.allocator,
                            ))
                        },
                        metadata: ClassDirectiveMetadata { dynamic: false },
                    })
                } else if directive_name == "let" {
                    Directive::Let(LetDirective {
                        span,
                        name,
                        expression: if let Some(expression) = expression {
                            match expression {
                                Expression::Identifier(identifier) => {
                                    Some(LetDirectiveExpression::Identifier(
                                        IdentifierName::new(
                                            identifier.span,
                                            identifier.name.clone(),
                                        ),
                                    ))
                                }
                                Expression::ArrayExpression(array) => Some(
                                    LetDirectiveExpression::ArrayExpression(
                                        array.unbox(),
                                    ),
                                ),
                                Expression::ObjectExpression(object) => Some(
                                    LetDirectiveExpression::ObjectExpression(
                                        object.unbox(),
                                    ),
                                ),
                                _ => {
                                    self.error(InvalidDirectiveValue(span));
                                    None
                                }
                            }
                        } else {
                            None
                        },
                    })
                } else if directive_name == "on" {
                    let mut modifiers = OxcVec::new_in(self.allocator);
                    for modifier in raw_modifiers {
                        modifiers.push(Atom::from(modifier));
                    }

                    Directive::On(OnDirective {
                        span,
                        name,
                        expression,
                        modifiers,
                    })
                } else if ["transition", "in", "out"]
                    .contains(&&directive_name.as_str())
                {
                    let mut modifiers = OxcVec::new_in(self.allocator);
                    for modifier in raw_modifiers {
                        if modifier == "local" {
                            modifiers.push(TransitionDirectiveModifier::Local);
                        } else if modifier == "global" {
                            modifiers.push(TransitionDirectiveModifier::Global);
                        } else {
                            // TODO: report error
                        }
                    }

                    Directive::Transition(TransitionDirective {
                        span,
                        name,
                        expression,
                        modifiers,
                        intro: directive_name == "in",
                        outro: directive_name == "out",
                    })
                } else if directive_name == "use" {
                    Directive::Use(UseDirective { span, name, expression })
                } else {
                    // TODO: report error
                    return None;
                },
            ));
        }

        Some(ElementAttribute::Attribute(Attribute {
            span: Span::new(start as u32, end as u32),
            name: Atom::from(name as &str),
            value,
        }))
    }

    fn parse_attribute_value(
        &mut self,
    ) -> OxcVec<'a, AttributeSequenceValue<'a>> {
        let quote_mark = if self.eat("\"", false) {
            Some('"')
        } else if self.eat("'", false) {
            Some('\'')
        } else {
            None
        };

        if let Some(quote_mark) = quote_mark {
            if self.eat(&quote_mark.to_string(), false) {
                let mut values = OxcVec::new_in(self.allocator);
                values.push(AttributeSequenceValue::Text(Text {
                    span: Span::new(
                        self.index as u32 - 1,
                        self.index as u32 - 1,
                    ),
                    data: Atom::from(""),
                    raw: Atom::from(""),
                }));
                return values;
            }
        }

        if let Some(sequence) =
            self.parse_sequence(quote_mark, "in attribute value")
        {
            sequence
        } else {
            // TODO: report error
            return OxcVec::new_in(self.allocator);
        }
    }

    fn parse_sequence(
        &mut self,
        quote_mark: Option<char>,
        location: &'static str,
    ) -> Option<OxcVec<'a, AttributeSequenceValue<'a>>> {
        let mut current_chunk = Text {
            span: Span::new(self.index as u32, 0),
            data: Atom::from(""),
            raw: Atom::from(""),
        };
        let mut chunks = OxcVec::new_in(self.allocator);

        while self.index < self.source_text.len() {
            let index = self.index;
            let done = if let Some(quote_mark) = quote_mark {
                self.match_str(&quote_mark.to_string())
            } else {
                self.match_regex(&REGEX_STARTS_WITH_INVALID_ATTR_VALUE)
                    .is_some_and(|mat| !mat.is_empty())
            };

            if done {
                current_chunk.span.end = self.index as u32;
                current_chunk.data = Atom::from(self.allocator.alloc(
                    decode_character_references(
                        current_chunk.data.as_str(),
                        true,
                    ),
                ) as &str);

                if !current_chunk.raw.as_str().is_empty() || chunks.is_empty() {
                    chunks.push(AttributeSequenceValue::Text(current_chunk));
                }

                return Some(chunks);
            } else if self.eat("{", false) {
                if self.match_str("#") {
                    self.eat("#", true);
                    let name =
                        self.read_until(&REGEX_NOT_LOWERCASE).to_string();
                    self.error(InvalidBlockPlacement(
                        Span::new(index as u32, self.index as u32),
                        location.to_string(),
                        name,
                    ));
                } else if self.match_str("@") {
                    self.eat("#", true);
                    let name =
                        self.read_until(&REGEX_NOT_LOWERCASE).to_string();
                    self.error(InvalidTagPlacement(
                        Span::new(index as u32, self.index as u32),
                        location.to_string(),
                        name,
                    ));
                }

                current_chunk.span.end = self.index as u32;
                current_chunk.data = Atom::from(self.allocator.alloc(
                    decode_character_references(
                        current_chunk.data.as_str(),
                        true,
                    ),
                ) as &str);

                if !current_chunk.raw.as_str().is_empty() {
                    chunks.push(AttributeSequenceValue::Text(current_chunk));
                }

                self.allow_whitespace();
                let expression = self.parse_expression();
                self.allow_whitespace();
                self.eat("}", true);

                if let Some(expression) = expression {
                    let chunk = ExpressionTag {
                        span: Span::new(index as u32, self.index as u32),
                        expression,
                        metadata: ExpressionTagMetadata {
                            contains_call_expression: false,
                            dynamic: false,
                        },
                    };

                    chunks.push(AttributeSequenceValue::ExpressionTag(chunk));
                } else {
                    // TODO: report error
                }

                current_chunk = Text {
                    span: Span::new(self.index as u32, self.index as u32),
                    data: Atom::from(""),
                    raw: Atom::from(""),
                };
            } else {
                current_chunk.raw = Atom::from(
                    &self.source_text
                        [(current_chunk.span.start as usize)..self.index],
                );
                self.index += 1;
            }
        }

        self.error(UnexpectedEof(Span::new(
            self.index as u32,
            self.index as u32,
        )));
        None
    }

    pub fn parse_static_attribute(&mut self) -> Option<Attribute<'a>> {
        let start = self.index;

        let name =
            self.read_until(&REGEX_TOKEN_ENDING_CHARACTER).trim().to_string();
        let name = self.allocator.alloc(name);

        if name.is_empty() {
            return None;
        }

        let mut value = AttributeValue::Bool(true);

        if self.eat("=", false) {
            self.allow_whitespace();

            let raw = self
                .match_regex(&REGEX_ATTRIBUTE_VALUE)
                .map(ToString::to_string);
            let raw = raw.map(|raw| self.allocator.alloc(raw));

            let raw = if let Some(raw) = raw {
                raw
            } else {
                let span = Span::new(self.index as u32, self.index as u32);
                self.error(MissingAttributeValue(span));
                return None;
            };

            self.index += raw.len();
            let first_char = raw.chars().next().unwrap();

            let quoted = match first_char {
                '"' => Some('"'),
                '\'' => Some('\''),
                _ => None,
            };

            let extracted =
                if quoted.is_some() { &raw[1..(raw.len() - 1)] } else { raw };

            let mut values = OxcVec::new_in(self.allocator);

            values.push(AttributeSequenceValue::Text(Text {
                span: Span::new(
                    (self.index
                        - extracted.len()
                        - if quoted.is_some() { 1 } else { 0 })
                        as u32,
                    (if quoted.is_some() { self.index - 1 } else { self.index })
                        as u32,
                ),
                data: Atom::from(
                    self.allocator
                        .alloc(decode_character_references(extracted, true))
                        as &str,
                ),
                raw: Atom::from(extracted),
            }));

            value = AttributeValue::Sequence(values);
        }

        if self.match_regex(&REGEX_STARTS_WITH_QUOTE_CHARACTER).is_some() {
            self.error(UnexpectedToken(
                Span::new(self.index as u32, self.index as u32),
                "=".to_string(),
            ));
        }

        Some(Attribute {
            span: Span::new(start as u32, self.index as u32),
            name: Atom::from(name as &str),
            value,
        })
    }
}
