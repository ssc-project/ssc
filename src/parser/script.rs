use std::collections::HashSet;

use lazy_static::lazy_static;
use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::Program;
use oxc_span::{SourceType, Span, SPAN};
use regex::Regex;

use super::{
    errors::parse::{InvalidScriptContext, UnclosedElement},
    Parser,
};
use crate::{
    ast::template::{
        AttributeSequenceValue, AttributeValue, Script, ScriptContext,
    },
    parser::errors::attributes::DuplicateAttribute,
};

lazy_static! {
    static ref REGEX_STARTS_CLOSING_SCRIPT_TAG: Regex =
        Regex::new(r"</script\s*>").unwrap();
}

impl<'a> Parser<'a> {
    pub fn parse_script(&mut self) -> Option<Script<'a>> {
        let start = self.index;
        if !self.eat("<", false) {
            return None;
        }

        let name = self.parse_tag_name();

        if name != "script" {
            self.index = start;
            return None;
        }

        let mut unique_names = HashSet::new();

        let mut context = ScriptContext::Default;
        let mut attributes = OxcVec::new_in(self.allocator);

        self.allow_whitespace();

        while let Some(attribute) = self.parse_static_attribute() {
            let attribute_name = attribute.name.as_str();
            if unique_names.contains(attribute_name) {
                self.error(DuplicateAttribute(attribute.span));
            } else {
                unique_names.insert(attribute.name.as_str().to_string());
            }

            if attribute_name == "context" {
                'context: {
                    if let AttributeValue::Sequence(values) = &attribute.value {
                        if let AttributeSequenceValue::Text(text) = &values[0] {
                            if text.data.as_str() == "module" {
                                context = ScriptContext::Module;
                                break 'context;
                            }
                        }
                    }

                    self.error(InvalidScriptContext(attribute.span));
                }
            }

            attributes.push(attribute);
            self.allow_whitespace();
        }

        self.eat(">", true);

        let program_start = self.index;
        self.read_until(&REGEX_STARTS_CLOSING_SCRIPT_TAG);
        let program_end = self.index;
        self.read(&REGEX_STARTS_CLOSING_SCRIPT_TAG);

        let content = if self.index >= self.source_text.len() {
            self.error(UnclosedElement(
                Span::new(
                    self.source_text.len() as u32,
                    self.source_text.len() as u32,
                ),
                "script".to_string(),
            ));
            Program {
                span: SPAN,
                source_type: SourceType::default().with_typescript(self.ts),
                directives: OxcVec::new_in(self.allocator),
                hashbang: None,
                body: OxcVec::new_in(self.allocator),
            }
        } else {
            let ret = crate::oxc_parser::Parser::new(
                self.allocator,
                &self.source_text[..program_end],
                SourceType::default().with_typescript(self.ts),
            )
            .parse_at_position(program_start);

            for error in ret.errors {
                self.error(error);
            }

            ret.program
        };

        Some(Script {
            span: Span::new(start as u32, start as u32),
            context,
            content,
            attributes,
        })
    }
}
