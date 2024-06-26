//! Token

use oxc_span::Span;

use super::kind::Kind;

#[derive(Debug, Clone, Copy, Default)]
pub struct Token {
    /// Token Kind
    pub kind: Kind,

    /// Start offset in source
    pub start: u32,

    /// End offset in source
    pub end: u32,

    /// Indicates the token is on a newline
    pub is_on_new_line: bool,

    /// True if the identifier / string / template kinds has escaped strings.
    /// The escaped strings are saved in [Lexer::escaped_strings] and
    /// [Lexer::escaped_templates] by [Token::start].
    ///
    /// [Lexer::escaped_strings]: [super::Lexer::escaped_strings]
    /// [Lexer::escaped_templates]: [super::Lexer::escaped_templates]
    pub escaped: bool,

    // Padding to fill to 16 bytes.
    // This makes copying a `Token` 1 x xmmword load & store, rather than 1 x
    // dword + 1 x qword and `Token::default()` is 1 x xmmword store,
    // rather than 1 x dword + 1 x qword.
    _padding2: u32,
}

#[cfg(target_pointer_width = "64")]
mod size_asserts {
    static_assertions::assert_eq_size!(super::Token, [u8; 16]);
}

impl Token {
    pub(super) fn new_on_new_line() -> Self {
        Self { is_on_new_line: true, ..Self::default() }
    }

    pub fn span(&self) -> Span {
        Span::new(self.start, self.end)
    }
}
