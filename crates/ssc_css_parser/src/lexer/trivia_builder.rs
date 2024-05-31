use oxc_span::Span;
use ssc_css_ast::{Comment, Trivias, TriviasMap};

#[derive(Debug, Default)]
pub struct TriviaBuilder {
    // Duplicated comments can be added from rewind, use `BTreeMap` to ensure
    // uniqueness
    comments: Vec<(u32, Comment)>,
    irregular_whitespaces: Vec<Span>,
}

impl TriviaBuilder {
    pub fn build(self) -> Trivias {
        let comments = TriviasMap::from_iter(self.comments);
        Trivias::new(comments, self.irregular_whitespaces)
    }

    pub fn add_comment(&mut self, start: u32, end: u32) {
        // The comments array is an ordered vec, only add the comment if its not
        // added before, to avoid situations where the parser needs to
        // rewind and reinsert the comment.
        if let Some(comment) = self.comments.last_mut() {
            if start <= comment.0 {
                return;
            }
        }
        // skip leading `/*` and trailing `*/`
        self.comments.push((start + 2, Comment::new(end - 2)));
    }

    pub fn add_irregular_whitespace(&mut self, start: u32, end: u32) {
        self.irregular_whitespaces.push(Span::new(start, end));
    }
}
