use napi_derive::napi;

use oxc_allocator::Allocator;
use oxc_diagnostics::Error;
pub use ssc_ast::ast::Root;
use ssc_parser::{Parser, ParserReturn};

#[napi(object)]
pub struct ParseResult {
    pub root: String,
    pub comments: Vec<Comment>,
    pub errors: Vec<String>,
}

#[napi(object)]
pub struct Comment {
    pub value: String,
    pub start: u32,
    pub end: u32,
}

fn parse<'a>(allocator: &'a Allocator, source_text: &'a str) -> ParserReturn<'a> {
    Parser::new(allocator, source_text).parse()
}

/// Parse without returning anything.
/// This is for benchmark purposes such as measuring napi communication overhead.
///
/// # Panics
///
/// * File extension is invalid
/// * Serde JSON serialization
#[allow(clippy::needless_pass_by_value)]
#[napi]
pub fn parse_without_return(source_text: String) {
    let allocator = Allocator::default();
    parse(&allocator, &source_text);
}

/// # Panics
///
/// * File extension is invalid
/// * Serde JSON serialization
#[allow(clippy::needless_pass_by_value)]
#[napi]
pub fn parse_sync(source_text: String) -> ParseResult {
    let allocator = Allocator::default();
    let ret = parse(&allocator, &source_text);
    let root = serde_json::to_string(&ret.root).unwrap();

    let errors = if ret.errors.is_empty() {
        vec![]
    } else {
        ret.errors
            .into_iter()
            .map(|diagnostic| Error::from(diagnostic).with_source_code(source_text.clone()))
            .map(|error| format!("{error:?}"))
            .collect()
    };

    let comments = ret
        .trivias
        .comments()
        .map(|span| Comment {
            value: span.source_text(&source_text).to_string(),
            start: span.start,
            end: span.end,
        })
        .collect::<Vec<Comment>>();

    ParseResult { root, comments, errors }
}

/// # Panics
///
/// * Tokio crashes
#[allow(clippy::needless_pass_by_value)]
#[napi]
pub async fn parse_async(source_text: String) -> ParseResult {
    tokio::spawn(async move { parse_sync(source_text) }).await.unwrap()
}
