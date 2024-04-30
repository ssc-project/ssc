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
mod utils;

use std::collections::HashSet;

use lazy_static::lazy_static;
use oxc_allocator::{Allocator, Vec as OxcVec};
use oxc_ast::ast::IdentifierName;
use oxc_diagnostics::Error;
use oxc_span::{Atom, GetSpan, Span, SPAN};
use oxc_syntax::identifier::{is_identifier_part, is_identifier_start};
use regex::Regex;

use self::{
    errors::parse::{
        DuplicateScriptElement, DuplicateStyleElement, MissingWhitespace,
        UnexpectedEof, UnexpectedEofWithExpected, UnexpectedReservedWord,
        UnexpectedToken,
    },
    names::RESERVED,
    patterns::REGEX_WHITESPACE,
    utils::full_char_code_at,
};
use crate::ast::template::{
    Fragment, FragmentNodeKind, Root, RootMetadata, ScriptContext,
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
    pub is_valid_for_self: bool,
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
            self.allow_whitespace();
            println!("---------------------------------------");
            println!("nodes: {:#?}", nodes);
            // println!("css: {:#?}", css);
            // println!("instance: {:#?}", instance);
            // println!("module: {:#?}", module);
            println!("---------------------------------------");
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
            options: None,
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
        &self.source_text[self.index..(self.index + str.len())] == str
    }

    fn match_regex(&self, reg: &Regex) -> Option<&str> {
        reg.find(&self.source_text[self.index..]).map(|mat| mat.as_str())
    }

    fn allow_whitespace(&mut self) {
        println!(
            "Checking for whitespace: \"{}\"",
            &self.source_text[self.index..(self.index + 1)]
        );
        while self.index < self.source_text.len()
            && REGEX_WHITESPACE
                .is_match(&self.source_text[self.index..(self.index + 1)])
        {
            println!(
                "Skipping: \"{}\"",
                &self.source_text[self.index..(self.index + 1)]
            );
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

    fn parse_identifier(
        &mut self,
        allow_reserved: bool,
    ) -> Option<IdentifierName<'a>> {
        let start = self.index;
        let mut i = self.index;
        let code = full_char_code_at(self.source_text, self.index);
        if !is_identifier_start(char::from_u32(code).unwrap()) {
            return None;
        }

        i += if code <= 0xffff { 1 } else { 2 };

        while i < self.source_text.len() {
            let code = full_char_code_at(self.source_text, i);

            if !is_identifier_part(char::from_u32(code).unwrap()) {
                break;
            }

            i += if code <= 0xffff { 1 } else { 2 };
        }

        self.index = i;

        let ident_name = &self.source_text[start..i];
        let span = Span::new(start as u32, i as u32);

        if !allow_reserved && RESERVED.contains(ident_name) {
            self.error(UnexpectedReservedWord(span, ident_name.to_string()));
            return None;
        }
        Some(IdentifierName { span, name: Atom::from(ident_name) })
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

        println!("Mat: {:#?}", mat);

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
        println!("Skipping whitespaces and comments");
        self.allow_whitespace();
        while self.match_str("/*") || self.match_str("<!--") {
            println!("comment");
            if self.eat("/*", false) {
                println!("js comment");
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
