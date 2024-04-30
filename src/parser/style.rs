use std::{collections::HashSet, fmt::Write};

use lazy_static::lazy_static;
use oxc_allocator::{Allocator, Vec as OxcVec};
use oxc_span::{Atom, Span};
use regex::Regex;

use super::{
    errors::{
        css::{InvalidCssDeclaration, InvalidCssSelector},
        parse::UnexpectedEof,
    },
    Parser,
};
use crate::{
    ast::css::{
        Atrule, AtruleOrRule, AttributeSelector, Block, BlockChild,
        ClassSelector, Combinator, ComplexSelector, ComplexSelectorMetadata,
        Declaration, IdSelector, NestingSelector, NestingSelectorName, Nth,
        Percentage, PseudoClassSelector, PseudoElementSelector,
        RelativeSelector, RelativeSelectorMetadata, Rule, RuleMetadata,
        SelectorList, SimpleSelector, StyleSheet, StyleSheetContent,
        TypeSelector,
    },
    parser::errors::{
        attributes::DuplicateAttribute, css::InvalidCssIdentifier,
    },
};

lazy_static! {
    static ref REGEX_STARTS_CLOSING_STYLE: Regex =
        Regex::new(r"^</style\s*>").unwrap();
    static ref REGEX_LEADING_HYPHEN_OR_DIGIT: Regex =
        Regex::new(r"^-?\d").unwrap();
    static ref REGEX_VALID_IDENTIFIER_CHAR: Regex =
        Regex::new("[a-zA-Z0-9_-]").unwrap();
    static ref REGEX_MATCHER: Regex = Regex::new(r"^[~^$*|]?=").unwrap();
    static ref REGEX_ATTRIBUTE_FLAGS: Regex =
        Regex::new(r"^[a-zA-Z]+").unwrap();
    static ref REGEX_NTH_OF: Regex = Regex::new(r#"(?x)^(even|odd|\+?(?:\d+|\d*n(?:\s*[+-]\s*\d+)?)|-\d*n(?:\s*\+\s*\d+))(?:\s*[,)]|\s+of\s+|$)"#).unwrap();
    static ref REGEX_PERCENTANGE: Regex = Regex::new(r#"^\d+(\.\d+)?%"#).unwrap();
    static ref REGEX_COMBINATOR: Regex = Regex::new(r#"^(\+|~|>||\|\|)"#).unwrap();
    static ref REGEX_CLOSING_BRACKET: Regex = Regex::new(r"[\s\]]").unwrap();
    static ref REGEX_WHITESPACE_OR_COLON: Regex = Regex::new(r"[\s:]").unwrap();
}

impl<'a> Parser<'a> {
    pub fn parse_style(&mut self) -> Option<StyleSheet<'a>> {
        let start = self.index;
        if !self.eat("<", false) {
            return None;
        }

        let name = self.parse_tag_name();

        if name != "style" {
            self.index = start;
            return None;
        }

        let mut unique_names = HashSet::new();

        let mut attributes = OxcVec::new_in(self.allocator);

        self.allow_whitespace();

        while let Some(attribute) = self.parse_static_attribute() {
            let attribute_name = attribute.name.as_str();
            if unique_names.contains(attribute_name) {
                self.error(DuplicateAttribute(attribute.span));
            } else {
                unique_names.insert(attribute.name.as_str().to_string());
            }

            attributes.push(attribute);
            self.allow_whitespace();
        }

        self.eat(">", true);
        let content_start = self.index;

        let mut children = OxcVec::new_in(self.allocator);

        while self.index < self.source_text.len() {
            self.allow_comment_or_whitespace();

            if self.match_regex(&REGEX_STARTS_CLOSING_STYLE).is_some() {
                break;
            }

            if self.match_str("@") {
                children.push(AtruleOrRule::Atrule(self.parse_css_at_rule()));
            } else {
                children.push(AtruleOrRule::Rule(self.parse_css_rule()));
            }
        }

        let content_end = self.index;

        self.read(&REGEX_STARTS_CLOSING_STYLE);

        Some(StyleSheet {
            span: Span::new(start as u32, self.index as u32),
            attributes,
            children,
            content: StyleSheetContent {
                span: Span::new(content_start as u32, content_end as u32),
                styles: Atom::from(
                    &self.source_text[content_start..content_end],
                ),
                comment: None,
            },
        })
    }

    fn parse_css_at_rule(&mut self) -> Atrule<'a> {
        let start = self.index;
        self.eat("@", true);

        let name = self.parse_css_identifier();
        let prelude = self.parse_css_value();

        let block = if self.match_str("{") {
            Some(self.parse_css_block())
        } else {
            self.eat(";", true);
            None
        };

        Atrule {
            span: Span::new(start as u32, self.index as u32),
            name,
            prelude,
            block,
        }
    }

    fn parse_css_rule(&mut self) -> Rule<'a> {
        let start = self.index;

        Rule {
            span: Span::new(start as u32, self.index as u32),
            prelude: self.parse_css_selector_list(false),
            block: self.parse_css_block(),
            metadata: RuleMetadata {
                has_local_selectors: false,
                is_global_block: false,
            },
        }
    }

    fn parse_css_selector_list(
        &mut self,
        inside_pseudo_class: bool,
    ) -> SelectorList<'a> {
        let mut children = OxcVec::new_in(self.allocator);
        self.allow_comment_or_whitespace();
        let start = self.index;

        while self.index < self.source_text.len() {
            children.push(self.parse_css_selector(inside_pseudo_class));

            let end = self.index;

            self.allow_whitespace();

            let return_condition =
                self.match_str(if inside_pseudo_class { ")" } else { "{" });

            if return_condition {
                return SelectorList {
                    span: Span::new(start as u32, end as u32),
                    children,
                };
            } else {
                self.eat(",", true);
                self.allow_comment_or_whitespace();
            }
        }

        SelectorList {
            span: Span::new(start as u32, self.index as u32),
            children,
        }
    }

    fn parse_css_selector(
        &mut self,
        inside_pseudo_class: bool,
    ) -> ComplexSelector<'a> {
        let list_start = self.index;
        let mut children = OxcVec::new_in(self.allocator);

        fn create_selector<'a>(
            allocator: &'a Allocator,
            combinator: Option<Combinator<'a>>,
            start: u32,
        ) -> RelativeSelector<'a> {
            RelativeSelector {
                span: Span::new(start, 0),
                combinator,
                selectors: OxcVec::new_in(allocator),
                metadata: RelativeSelectorMetadata {
                    is_global: false,
                    is_host: false,
                    root: false,
                    scoped: false,
                },
            }
        }

        let mut relative_selector =
            create_selector(self.allocator, None, self.index as u32);

        while self.index < self.source_text.len() {
            let start = self.index;

            if self.eat("&", false) {
                relative_selector.selectors.push(
                    SimpleSelector::NestingSelector(NestingSelector {
                        span: Span::new(start as u32, self.index as u32),
                        name: NestingSelectorName, // "&"
                    }),
                );
            } else if self.eat("*", false) {
                let mut name = Atom::from("*");

                if self.eat("|", false) {
                    name = self.parse_css_identifier();
                }

                relative_selector.selectors.push(SimpleSelector::TypeSelector(
                    TypeSelector {
                        span: Span::new(start as u32, self.index as u32),
                        name,
                    },
                ));
            } else if self.eat("#", false) {
                relative_selector.selectors.push(SimpleSelector::IdSelector(
                    IdSelector {
                        span: Span::new(start as u32, self.index as u32),
                        name: self.parse_css_identifier(),
                    },
                ));
            } else if self.eat(".", false) {
                relative_selector.selectors.push(
                    SimpleSelector::ClassSelector(ClassSelector {
                        span: Span::new(start as u32, self.index as u32),
                        name: self.parse_css_identifier(),
                    }),
                );
            } else if self.eat("::", false) {
                relative_selector.selectors.push(
                    SimpleSelector::PseudoElementSelector(
                        PseudoElementSelector {
                            span: Span::new(start as u32, self.index as u32),
                            name: self.parse_css_identifier(),
                        },
                    ),
                );

                if self.eat("(", false) {
                    self.parse_css_selector_list(true);
                    self.eat(")", true);
                }
            } else if self.eat(":", false) {
                let name = self.parse_css_identifier();

                let args = if self.eat("(", false) {
                    let args = self.parse_css_selector_list(true);
                    self.eat(")", true);
                    Some(args)
                } else {
                    None
                };

                relative_selector.selectors.push(
                    SimpleSelector::PseudoClassSelector(PseudoClassSelector {
                        span: Span::new(start as u32, self.index as u32),
                        name,
                        args,
                    }),
                )
            } else if self.eat("[", false) {
                self.allow_whitespace();
                let name = self.parse_css_identifier();
                self.allow_whitespace();

                let matcher = self.read(&REGEX_MATCHER).map(|matcher| {
                    Atom::from(self.allocator.alloc(matcher) as &str)
                });

                let value = if matcher.is_some() {
                    self.allow_whitespace();
                    Some(self.parse_css_attribute_value())
                } else {
                    None
                };

                self.allow_whitespace();

                let flags = self.read(&REGEX_ATTRIBUTE_FLAGS).map(|matcher| {
                    Atom::from(self.allocator.alloc(matcher) as &str)
                });

                self.allow_whitespace();
                self.eat("]", true);

                relative_selector.selectors.push(
                    SimpleSelector::AttributeSelector(AttributeSelector {
                        span: Span::new(start as u32, self.index as u32),
                        name,
                        matcher,
                        value,
                        flags,
                    }),
                );
            } else if inside_pseudo_class
                && self.match_regex(&REGEX_NTH_OF).is_some()
            {
                relative_selector.selectors.push(SimpleSelector::Nth(Nth {
                    span: Span::new(start as u32, self.index as u32),
                    value: Atom::from(
                        self.allocator.alloc(self.read(&REGEX_NTH_OF).unwrap())
                            as &str,
                    ),
                }));
            } else if self.match_regex(&REGEX_PERCENTANGE).is_some() {
                relative_selector.selectors.push(SimpleSelector::Percentage(
                    Percentage {
                        span: Span::new(start as u32, self.index as u32),
                        value: Atom::from(
                            self.allocator
                                .alloc(self.read(&REGEX_PERCENTANGE).unwrap())
                                as &str,
                        ),
                    },
                ));
            } else {
                let mat = self.match_regex(&REGEX_COMBINATOR);
                let type_condition =
                    if let Some(mat) = mat { mat.is_empty() } else { true };
                if type_condition {
                    let name = self.parse_css_identifier();

                    let name = if self.eat("|", false) {
                        self.parse_css_identifier()
                    } else {
                        name
                    };

                    relative_selector.selectors.push(
                        SimpleSelector::TypeSelector(TypeSelector {
                            span: Span::new(start as u32, self.index as u32),
                            name,
                        }),
                    );
                }
            }

            let index = self.index;
            self.allow_whitespace();

            if self.match_str(",")
                || (inside_pseudo_class && self.match_str(")"))
                || (!inside_pseudo_class && self.match_str("{"))
            {
                self.index = index;
                relative_selector.span.end = index as u32;
                children.push(relative_selector);

                return ComplexSelector {
                    span: Span::new(list_start as u32, self.index as u32),
                    children,
                    metadata: ComplexSelectorMetadata { used: false },
                };
            }

            self.index = index;
            let combinator = self.parse_css_combinator();

            if let Some(combinator) = combinator {
                if relative_selector.selectors.is_empty() {
                    if !inside_pseudo_class {
                        self.error(InvalidCssSelector(Span::new(
                            start as u32,
                            start as u32,
                        )));
                    }
                } else {
                    relative_selector.span.end = index as u32;
                    children.push(relative_selector);
                }

                let selector_start = combinator.span.start;

                relative_selector = create_selector(
                    self.allocator,
                    Some(combinator),
                    selector_start,
                );

                self.allow_whitespace();

                if self.match_str(",")
                    || (inside_pseudo_class && self.match_str(")"))
                    || (!inside_pseudo_class && self.match_str("{"))
                {
                    self.error(InvalidCssSelector(Span::new(
                        self.index as u32,
                        self.index as u32,
                    )));
                }
            }
        }

        self.error(UnexpectedEof(Span::new(
            self.index as u32,
            self.index as u32,
        )));

        ComplexSelector {
            span: Span::new(list_start as u32, self.index as u32),
            children,
            metadata: ComplexSelectorMetadata { used: false },
        }
    }

    fn parse_css_combinator(&mut self) -> Option<Combinator<'a>> {
        let start = self.index;
        self.allow_whitespace();

        let index = self.index;
        let name = self.read(&REGEX_COMBINATOR);

        if let Some(name) = name {
            let end = self.index;
            self.allow_whitespace();

            Some(Combinator {
                span: Span::new(index as u32, end as u32),
                name: Atom::from(self.allocator.alloc(name) as &str),
            })
        } else if self.index != start {
            Some(Combinator {
                span: Span::new(start as u32, self.index as u32),
                name: Atom::from(" "),
            })
        } else {
            None
        }
    }

    fn parse_css_identifier(&mut self) -> Atom<'a> {
        let start = self.index;

        let mut identifier = String::new();

        if self.match_str("--")
            || self.match_regex(&REGEX_LEADING_HYPHEN_OR_DIGIT).is_some()
        {
            self.error(InvalidCssIdentifier(Span::new(
                start as u32,
                start as u32,
            )));
        }

        let mut escaped = false;

        while self.index < self.source_text.len() {
            let ch: char = self.source_text.as_bytes()[self.index].into();

            if escaped {
                write!(identifier, "\\{}", ch).unwrap();
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch as u32 >= 160
                || REGEX_VALID_IDENTIFIER_CHAR.is_match(&format!("{}", ch))
            {
                write!(identifier, "{}", ch).unwrap();
            } else {
                break;
            }

            self.index += 1;
        }

        Atom::from(self.allocator.alloc(identifier) as &str)
    }

    fn parse_css_value(&mut self) -> Atom<'a> {
        let mut value = String::new();
        let mut escaped = false;
        let mut in_url = false;

        let mut quote_mark = None;

        while self.index < self.source_text.len() {
            let ch: char = self.source_text.as_bytes()[self.index].into();

            // TODO: there's some confusion
            if escaped {
                write!(value, "\\{}", ch).unwrap();
                escaped = false;
                self.index += 1;
                continue;
            } else if ch == '\\' {
                escaped = true;
                self.index += 1;
                continue;
            } else if quote_mark == Some(ch) {
                quote_mark = None;
            } else if ch == ')' {
                in_url = false;
            } else if quote_mark.is_none() && (ch == '"' || ch == '\'') {
                quote_mark = Some(ch);
            } else if ch == '(' && &value[(value.len().max(3) - 3)..] == "url" {
                in_url = true;
            } else if (ch == ';' || ch == '{' || ch == '}')
                && !in_url
                && quote_mark.is_none()
            {
                return Atom::from(
                    self.allocator.alloc(value.trim().to_string()) as &str,
                );
            }

            write!(value, "{}", ch).unwrap();

            self.index += 1;
        }

        self.error(UnexpectedEof(Span::new(
            self.source_text.len() as u32,
            self.source_text.len() as u32,
        )));

        Atom::from(self.allocator.alloc(value.trim().to_string()) as &str)
    }

    fn parse_css_attribute_value(&mut self) -> Atom<'a> {
        let mut value = String::new();
        let mut escaped = false;
        let quote_mark = if self.eat("\"", false) {
            Some('"')
        } else if self.eat("'", false) {
            Some('\'')
        } else {
            None
        };

        while self.index < self.source_text.len() {
            let ch: char = self.source_text.as_bytes()[self.index].into();

            if escaped {
                write!(value, "\\{}", ch).unwrap();
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if Some(ch) == quote_mark
                || REGEX_CLOSING_BRACKET.is_match(&ch.to_string())
            {
                if let Some(quote_mark) = quote_mark {
                    self.eat(&quote_mark.to_string(), true);
                }

                return Atom::from(
                    self.allocator.alloc(value.trim().to_string()) as &str,
                );
            } else {
                write!(value, "{}", ch).unwrap();
            }

            self.index += 1;
        }

        self.error(UnexpectedEof(Span::new(
            self.index as u32,
            self.index as u32,
        )));

        Atom::from(self.allocator.alloc(value.trim().to_string()) as &str)
    }

    fn parse_css_block(&mut self) -> Block<'a> {
        let start = self.index;
        self.eat("{", true);

        let mut children = OxcVec::new_in(self.allocator);

        while self.index < self.source_text.len() {
            self.allow_comment_or_whitespace();

            if self.match_str("}") {
                break;
            } else {
                children.push(self.parse_css_block_item());
            }
        }

        self.eat("}", true);

        Block { span: Span::new(start as u32, self.index as u32), children }
    }

    fn parse_css_block_item(&mut self) -> BlockChild<'a> {
        if self.match_str("@") {
            return BlockChild::Atrule(self.parse_css_at_rule());
        }

        let start = self.index;
        self.parse_css_value();
        let ch: char = self.source_text.as_bytes()[self.index].into();
        self.index = start;

        if ch == '{' {
            BlockChild::Rule(self.parse_css_rule())
        } else {
            BlockChild::Declaration(self.parse_css_declaration())
        }
    }

    fn parse_css_declaration(&mut self) -> Declaration<'a> {
        let start = self.index;

        let property = self
            .allocator
            .alloc(self.read_until(&REGEX_WHITESPACE_OR_COLON).to_string());
        self.allow_whitespace();
        self.eat(":", false);
        self.allow_whitespace();

        let value = self.parse_css_value();

        if value.is_empty() && !property.starts_with("--") {
            self.error(InvalidCssDeclaration(Span::new(
                self.index as u32,
                self.index as u32,
            )));
        }

        let end = self.index;

        if !self.match_str("}") {
            self.eat(";", true);
        }

        Declaration {
            span: Span::new(start as u32, end as u32),
            property: Atom::from(property as &str),
            value,
        }
    }
}
