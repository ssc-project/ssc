mod block;
mod comment;
mod element;
mod errors;
mod names;
mod patterns;
mod script;
mod style;
mod tag;
mod text;
pub(crate) mod utils;
use std::collections::HashSet;

use lazy_static::lazy_static;
use oxc_allocator::{Allocator, Vec as OxcVec};
use oxc_ast::ast::{
    Expression, IdentifierName, ObjectPropertyKind, PropertyKey,
};
use oxc_diagnostics::{Error, Report};
use oxc_span::{GetSpan, SourceType, Span, SPAN};
use regex::Regex;
use rustc_hash::FxHashMap;

use self::{
    errors::parse::{
        DuplicateScriptElement, DuplicateStyleElement, MissingWhitespace,
        UnexpectedEof, UnexpectedEofWithExpected, UnexpectedToken,
    },
    patterns::REGEX_WHITESPACE,
    utils::copy_ast_node,
};
use crate::ast::template::{
    AttributeSequenceValue, AttributeValue, CustomElement, CustomElementExtend,
    CustomElementProp, CustomElementPropType, CustomElementShadow,
    ElementAttribute, ElementLike, Fragment, FragmentNodeKind, Namespace, Root,
    RootMetadata, ScriptContext, SvelteOptions,
};

lazy_static! {
    static ref REGEX_LANG_ATTRIBUTE: Regex = Regex::new(r#"(?m)<!--[^>]*?-->|<script\s+(?:[^>]*\s+)?lang=(?:"([^"]*)"|'([^']*)')\s*(?:[^>]*)>"#).unwrap();
    static ref REGEX_COMMENT_CLOSE: Regex = Regex::new(r"\*/").unwrap();
    static ref REGEX_HTML_COMMENT_CLOSE: Regex = Regex::new("-->").unwrap();
}

pub struct ParserReturn<'a> {
    pub root: Root<'a>,
    pub errors: Vec<Error>,
}

#[derive(Default)]
struct ParserState {
    pub inside_head: bool,
    pub is_inside_if_block: bool,
    pub is_inside_each_block: bool,
    pub is_inside_snippet_block: bool,
}

pub struct Parser<'a> {
    allocator: &'a Allocator,
    source_text: &'a str,
    errors: Vec<Error>,
    #[allow(unused)]
    index: usize,
    ts: bool,
    state: ParserState,
    meta_tags: HashSet<String>,
}

impl<'a> Parser<'a> {
    pub fn new(allocator: &'a Allocator, source_text: &'a str) -> Self {
        let ts = 'ts: {
            for captures in REGEX_LANG_ATTRIBUTE.captures_iter(source_text) {
                if captures.get(0).unwrap().as_str().starts_with("<s") {
                    let lang = captures.get(1).unwrap().as_str();
                    if lang == "ts" {
                        break 'ts true;
                    }
                }
            }
            false
        };

        Self {
            allocator,
            source_text: source_text.trim_end(),
            errors: Vec::new(),
            index: 0,
            ts,
            meta_tags: HashSet::new(),
            state: ParserState::default(),
        }
    }

    pub fn parse(mut self) -> ParserReturn<'a> {
        let root = self.parse_root();
        ParserReturn { root, errors: self.errors }
    }

    fn error<E: Into<Error>>(&mut self, error: E) {
        self.errors.push(error.into());
    }

    fn parse_root(&mut self) -> Root<'a> {
        let mut nodes = OxcVec::new_in(self.allocator);
        let mut css = None;
        let mut instance = None;
        let mut module = None;

        while self.index < self.source_text.len() {
            if let Some(comment) = self.parse_comment() {
                nodes.push(FragmentNodeKind::Comment(comment));
                continue;
            }

            if let Some(script) = self.parse_script() {
                if script.context == ScriptContext::Module {
                    if module.is_some() {
                        self.error(DuplicateScriptElement(script.span));
                        continue;
                    }
                    module = Some(script);
                    continue;
                }
                if instance.is_some() {
                    self.error(DuplicateScriptElement(script.span));
                    continue;
                }
                instance = Some(script);
                continue;
            }

            if let Some(style) = self.parse_style() {
                if css.is_some() {
                    self.error(DuplicateStyleElement(style.span));
                    continue;
                }
                css = Some(style);
                continue;
            }

            if let Some(element_like) = self.parse_element_like(true) {
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

        let mut span = if nodes.is_empty() {
            SPAN
        } else {
            Span::new(nodes[0].span().start, nodes[nodes.len() - 1].span().end)
        };

        if let Some(css) = &css {
            if css.span.start < span.start {
                span.start = css.span.start;
            }
            if css.span.end > span.end {
                span.end = css.span.end;
            }
        }

        let options_index = nodes.iter().position(|node| {
            matches!(
                node,
                FragmentNodeKind::ElementLike(ElementLike::SvelteOptionsRaw(_))
            )
        });

        let options = if let Some(options_index) = options_index {
            let node = nodes.remove(options_index);

            if let FragmentNodeKind::ElementLike(
                ElementLike::SvelteOptionsRaw(raw_options),
            ) = node
            {
                let mut runes = None;
                let mut immutable = None;
                let mut accessors = None;
                let mut preserve_whitespace = None;
                let mut namespace = None;
                let mut custom_element = None;
                let mut attributes = OxcVec::new_in(self.allocator);

                for attribute in raw_options.attributes {
                    if let ElementAttribute::Attribute(attribute) = attribute {
                        match attribute.name.as_str() {
                            "runes" => {
                                if let AttributeValue::Bool(value) =
                                    attribute.value
                                {
                                    runes = Some(value);
                                } else {
                                    // TODO: report error
                                }
                            }
                            "customElement" => {
                                'custom_element: {
                                    if let AttributeValue::Sequence(seq) =
                                        &attribute.value
                                    {
                                        if let Some(
                                        AttributeSequenceValue::ExpressionTag(
                                            tag,
                                        ),
                                    ) = seq.first()
                                    {
                                        if let Expression::ObjectExpression(
                                            object,
                                        ) = &tag.expression
                                        {
                                            let mut properties = Vec::new();

                                            for property in &object.properties {
                                                if let ObjectPropertyKind::ObjectProperty(property) = property {
                                                    if let PropertyKey::Identifier(key) = &property.key {
                                                        properties.push((key.name.clone(), copy_ast_node(&property.value)));
                                                    } else {
                                                        // TODO: report error
                                                    }
                                                } else {
                                                    // TODO: report error
                                                }
                                            }

                                            let tag_expression = properties
                                                .iter()
                                                .find(|(key, _)| {
                                                    key.as_str() == "tag"
                                                })
                                                .map(|(_, value)| value);

                                            let tag = if let Some(Expression::StringLiteral(literal)) = tag_expression {
                                                literal.value.clone()
                                            } else {
                                                // TODO: report error
                                                break 'custom_element;
                                            };

                                            let mut props = FxHashMap::default();

                                            let props_expression = properties.iter().find(|(key, _)| key.as_str() == "props").map(|(_, value)| value);

                                            if let Some(expression) = props_expression {
                                                if let Expression::ObjectExpression(object) = expression {
                                                    for property in &object.properties {
                                                        if let ObjectPropertyKind::ObjectProperty(property) = property {
                                                            if let PropertyKey::Identifier(key) = &property.key {
                                                            if let Expression::ObjectExpression(object) = &property.value {
                                                                let mut attribute = None;
                                                                let mut reflect = None;
                                                                let mut type_ = None;
                                                                for property in &object.properties {
                                                                    if let ObjectPropertyKind::ObjectProperty(property) = property {
                                                                        if let PropertyKey::Identifier(prop_name) = &property.key {
                                                                            if prop_name.name.as_str() == "type" {
                                                                                if let Expression::StringLiteral(literal) = &property.value {
                                                                                    let value = match literal.value.as_str() {
                                                                                        "String" => Some(CustomElementPropType::String),
                                                                                        "Number" => Some(CustomElementPropType::Number),
                                                                                        "Boolean" => Some(CustomElementPropType::Boolean),
                                                                                        "Array" => Some(CustomElementPropType::Array),
                                                                                        "Object" => Some(CustomElementPropType::Object),
                                                                                        _ => None
                                                                                    };

                                                                                    if let Some(value) = value {
                                                                                        type_ = Some(value);
                                                                                    } else {
                                                                                        // TODO: report error
                                                                                    }
                                                                                } else {
                                                                                    // TODO: report error
                                                                                }
                                                                            } else if prop_name.name.as_str() == "reflect" {
                                                                                if let Expression::BooleanLiteral(literal) = &property.value {
                                                                                    reflect = Some(literal.value);
                                                                                }
                                                                            } else if prop_name.name.as_str() == "attribute" {
                                                                                if let Expression::StringLiteral(literal) = &property.value {
                                                                                    attribute = Some(literal.value.clone());
                                                                                }
                                                                            }
                                                                        } else {
                                                                            // TODO: report error
                                                                        }
                                                                    } else {
                                                                        // TODO: report error
                                                                    }
                                                                }
                                                                let prop = CustomElementProp {
                                                                    attribute,
                                                                    reflect,
                                                                    type_,
                                                                };
                                                                props.insert(key.name.clone(), prop);
                                                            } else {
                                                                // TODO: report error
                                                            }
                                                            } else {
                                                                // TODO: report error
                                                            }
                                                        } else {
                                                            // TODO: report error
                                                        }
                                                    }
                                                } else {
                                                    // TODO: report error
                                                }
                                            }

                                            let shadow_expression = properties.iter().find(|(key, _)| key.as_str() == "shadow").map(|(_, value)| value);

                                            let shadow = if let Some(shadow) = shadow_expression {
                                                if let Expression::StringLiteral(literal) = shadow {
                                                    if literal.value.as_str() == "open" {
                                                        Some(CustomElementShadow::Open)
                                                    } else if literal.value.as_str() == "none" {
                                                        Some(CustomElementShadow::None)
                                                    } else {
                                                        // TODO: report error
                                                        None
                                                    }
                                                } else {
                                                    // TODO: report error
                                                    None
                                                }
                                            } else {
                                                None
                                            };

                                            let extend_expression = properties.into_iter().find(|(key, _)| key.as_str() == "extend").map(|(_, value)| value);

                                            let extend = if let Some(extend) = extend_expression {
                                                if let Expression::ArrowFunctionExpression(arrow_function) = extend {
                                                    Some(CustomElementExtend::ArrowFunction(arrow_function.unbox()))
                                                } else if let Expression::Identifier(identifier) = extend {
                                                    Some(CustomElementExtend::Identifier(IdentifierName::new(identifier.span, identifier.unbox().name)))
                                                } else {
                                                    // TODO: report error
                                                    None
                                                }
                                            } else {
                                                None
                                            };

                                            custom_element =
                                                Some(CustomElement {
                                                    tag,
                                                    shadow,
                                                    props,
                                                    extend,
                                                })
                                        }
                                    }
                                    }
                                }
                            }
                            "namespace" => {
                                if let AttributeValue::Sequence(seq) =
                                    &attribute.value
                                {
                                    if let Some(AttributeSequenceValue::Text(
                                        text,
                                    )) = seq.first()
                                    {
                                        let value = match text.data.as_str() {
                                            "svg"
                                            | "http://www.w3.org/2000/svg" => {
                                                Some(Namespace::Svg)
                                            }
                                            "html" => Some(Namespace::Html),
                                            "foreign" => {
                                                Some(Namespace::Foreign)
                                            }
                                            _ => None,
                                        };

                                        if let Some(value) = value {
                                            namespace = Some(value);
                                        } else {
                                            // TODO: report error
                                        }
                                    } else {
                                        // TODO: report error
                                    }
                                } else {
                                    // TODO: report error
                                }
                            }
                            "immutable" => {
                                if let AttributeValue::Bool(value) =
                                    attribute.value
                                {
                                    immutable = Some(value);
                                } else {
                                    // TODO: report error
                                }
                            }
                            "preserveWhitespace" => {
                                if let AttributeValue::Bool(value) =
                                    attribute.value
                                {
                                    preserve_whitespace = Some(value);
                                } else {
                                    // TODO: report error
                                }
                            }
                            "accessors" => {
                                if let AttributeValue::Bool(value) =
                                    attribute.value
                                {
                                    accessors = Some(value);
                                } else {
                                    // TODO: report error
                                }
                            }
                            _ => {
                                // TODO: report error
                            }
                        };
                        attributes.push(attribute);
                    } else {
                        // TODO: report error
                    }
                }

                Some(SvelteOptions {
                    span: raw_options.span,
                    runes,
                    immutable,
                    accessors,
                    preserve_whitespace,
                    namespace,
                    custom_element,
                    attributes,
                })
            } else {
                None
            }
        } else {
            None
        };

        if let Some(instance) = &instance {
            if instance.span.start < span.start {
                span.start = instance.span.start;
            }
            if instance.span.end > span.end {
                span.end = instance.span.end;
            }
        }

        if let Some(module) = &module {
            if module.span.start < span.start {
                span.start = module.span.start;
            }
            if module.span.end > span.end {
                span.end = module.span.end;
            }
        }

        let fragment = Fragment { nodes, transparent: false };

        Root {
            span,
            options,
            fragment,
            css,
            instance,
            module,
            metadata: RootMetadata { ts: self.ts },
        }
    }

    fn eat(&mut self, str: &str, required: bool) -> bool {
        if self.match_str(str) {
            self.index += str.len();
            return true;
        }

        if required {
            if self.index == self.source_text.len() {
                self.error(UnexpectedEofWithExpected(
                    Span::new(self.index as u32, self.index as u32),
                    str.to_string(),
                ));
            } else {
                self.error(UnexpectedToken(
                    Span::new(self.index as u32, self.index as u32),
                    str.to_string(),
                ))
            }
        }
        false
    }

    fn match_str(&self, str: &str) -> bool {
        &self.source_text[self.index.min(self.source_text.len())
            ..(self.index + str.len()).min(self.source_text.len())]
            == str
    }

    fn match_regex(&self, reg: &Regex) -> Option<&str> {
        reg.find(&self.source_text[self.index..]).map(|mat| mat.as_str())
    }

    fn allow_whitespace(&mut self) {
        while self.index < self.source_text.len()
            && REGEX_WHITESPACE.is_match(
                &self.source_text[self.index.min(self.source_text.len())
                    ..(self.index + 1).min(self.source_text.len())],
            )
        {
            self.index += 1;
        }
    }

    fn read(&mut self, reg: &Regex) -> Option<String> {
        let result = self.match_regex(reg);
        if let Some(result) = result {
            let result = result.to_owned();
            self.index += result.len();
            return Some(result);
        }
        None
    }

    fn parse_identifier(&mut self) -> Result<IdentifierName<'a>, Report> {
        let parser = crate::oxc_parser::Parser::new(
            self.allocator,
            self.source_text,
            SourceType::default(),
        );
        let identifier = parser.parse_identifier_name_at(self.index);

        if let Ok(identifier) = &identifier {
            self.index = identifier.span.end as usize;
        }

        identifier
    }

    fn read_until(&mut self, reg: &Regex) -> &str {
        if self.index >= self.source_text.len() {
            self.error(UnexpectedEof(Span::new(
                self.source_text.len() as u32,
                self.source_text.len() as u32,
            )));
        }

        let start = self.index;
        let mat = reg.find(&self.source_text[self.index..]);

        if let Some(mat) = mat {
            self.index += mat.start();
            return &self.source_text[start..self.index];
        }

        self.index = self.source_text.len();
        &self.source_text[start..]
    }

    fn require_whitespace(&mut self) {
        if !REGEX_WHITESPACE.is_match(&self.source_text[self.index..]) {
            self.error(MissingWhitespace(Span::new(
                self.index as u32,
                self.index as u32,
            )));
        }

        self.allow_whitespace();
    }

    fn allow_comment_or_whitespace(&mut self) {
        self.allow_whitespace();
        while self.match_str("/*") || self.match_str("<!--") {
            if self.eat("/*", false) {
                self.read_until(&REGEX_COMMENT_CLOSE);
                self.eat("*/", true);
            } else {
                self.read_until(&REGEX_HTML_COMMENT_CLOSE);
                self.eat("-->", true);
            }

            self.allow_whitespace();
        }
    }
}

pub fn parse<'a>(
    allocator: &'a Allocator,
    source_text: &'a str,
) -> Result<Root<'a>, Vec<Error>> {
    let parser = Parser::new(allocator, source_text);
    let ret = parser.parse();
    if !ret.errors.is_empty() {
        return Err(ret.errors);
    }
    Ok(ret.root)
}
