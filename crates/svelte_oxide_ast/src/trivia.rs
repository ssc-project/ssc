//! Trivias such as comments and irregular whitespaces

use std::{
    collections::btree_map::{BTreeMap, Range},
    ops::RangeBounds,
};

use oxc_span::Span;

/// Single or multiline comment
#[derive(Debug, Clone, Copy)]
pub struct Comment {
    pub end: u32,
}

impl Comment {
    pub fn new(end: u32) -> Self {
        Self { end }
    }
}

pub type TriviasMap = BTreeMap<u32, Comment>;

#[derive(Debug, Default)]
pub struct Trivias {
    /// Keyed by span.start
    comments: TriviasMap,

    irregular_whitespaces: Vec<Span>,
}

impl Trivias {
    pub fn new(comments: TriviasMap, irregular_whitespaces: Vec<Span>) -> Self {
        Self { comments, irregular_whitespaces }
    }

    pub fn comments(&self) -> impl Iterator<Item = Span> + '_ {
        self.comments
            .iter()
            .map(|(start, comment)| Span::new(*start, comment.end))
    }

    pub fn comments_range<R>(&self, range: R) -> Range<'_, u32, Comment>
    where
        R: RangeBounds<u32>,
    {
        self.comments.range(range)
    }

    pub fn has_comments_between(&self, span: Span) -> bool {
        self.comments.range(span.start..span.end).count() > 0
    }

    pub fn irregular_whitespaces(&self) -> &Vec<Span> {
        &self.irregular_whitespaces
    }
}
