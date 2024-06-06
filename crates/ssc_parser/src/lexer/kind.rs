//! Token Kinds

use std::fmt;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub enum Kind {
    Undetermined,
    #[default]
    Eof,
    Skip, // Whitespace, line breaks, comments
    Hash,
    // identifier
    Ident,
    // keyword
    As,
    Await,
    Catch,
    Const,
    Debug,
    Each,
    Else,
    Html,
    If,
    Key,
    Render,
    Script,
    Snippet,
    Style,
    Then,
    // 12.8 punctuators
    Amp, // &
    Bang,
    Caret,
    Colon,
    Comma,
    Dot,
    Dot3, // ...
    Eq,
    LAngle,
    LBrack,
    LCurly,
    LParen,
    Minus,
    Percent,
    Pipe,
    Plus,
    Question,
    RAngle,
    RBrack,
    RCurly,
    RParen,
    Semicolon,
    Slash,
    Star,
    Tilde,
    Str,
    Backtick,
    At,
    // Number
    Number,
    // Text
    Text,
}

#[allow(clippy::enum_glob_use)]
use self::Kind::*;

impl Kind {
    pub fn is_eof(self) -> bool {
        matches!(self, Eof)
    }

    pub fn match_keyword(s: &str) -> Self {
        let len = s.len();
        if len <= 1 || len >= 12 || !s.as_bytes()[0].is_ascii_lowercase() {
            return Ident;
        }
        Self::match_keyword_impl(s)
    }

    fn match_keyword_impl(s: &str) -> Self {
        match s {
            "as" => As,
            "await" => Await,
            "catch" => Catch,
            "const" => Const,
            "each" => Each,
            "else" => Else,
            "html" => Html,
            "if" => If,
            "key" => Key,
            "render" => Render,
            "script" => Script,
            "snippet" => Snippet,
            "style" => Style,
            "then" => Then,
            _ => Ident,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Undetermined => "Unknown",
            Eof => "EOF",
            Skip => "Skipped",
            Hash => "#",
            Ident => "Identifier",
            As => "as",
            Await => "await",
            Catch => "catch",
            Const => "const",
            Debug => "debug",
            Each => "each",
            Else => "else",
            Html => "html",
            If => "if",
            Key => "key",
            Render => "render",
            Script => "script",
            Snippet => "snippet",
            Style => "style",
            Then => "then",
            Amp => "&",
            Bang => "!",
            Caret => "^",
            Colon => ":",
            Comma => ",",
            Dot => ".",
            Dot3 => "...",
            Eq => "=",
            LAngle => "<",
            LBrack => "[",
            LCurly => "{",
            LParen => "(",
            Minus => "-",
            Percent => "%",
            Pipe => "|",
            Plus => "+",
            Question => "?",
            RAngle => ">",
            RBrack => "]",
            RCurly => "}",
            RParen => ")",
            Semicolon => ";",
            Slash => "/",
            Star => "*",
            Tilde => "~",
            Str => "string",
            Backtick => "`",
            At => "@",
            Number => "number",
            Text => "text",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
