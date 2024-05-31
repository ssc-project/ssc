//! ECMAScript Token Kinds

use std::fmt;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub enum Kind {
    Undetermined,
    #[default]
    Eof,
    Skip, // Whitespace, line breaks, comments
    Hash,
    Ident,
    Of,
    Url,
    Even,
    Odd,
    N,
    // 12.8 punctuators
    Amp, // &
    Bang,
    Caret,
    Colon,
    Colon2,
    Comma,
    Dot,
    Eq,
    LAngle,
    LBrack,
    LCurly,
    LParen,
    Minus,
    Percent,
    Pipe,
    Pipe2,
    Plus,
    RAngle,
    RBrack,
    RCurly,
    RParen,
    Semicolon,
    Star,
    Tilde,
    Dollar,
    /// String Type
    Str,
    At,
    Slash,
    Number,
    Question,
    Backtick,
}

#[allow(clippy::enum_glob_use)]
use self::Kind::*;

impl Kind {
    pub fn is_eof(self) -> bool {
        matches!(self, Eof)
    }

    pub fn match_keyword(s: &str) -> Self {
        let len = s.len();
        if len == 0 || len >= 12 || !s.as_bytes()[0].is_ascii_lowercase() {
            return Ident;
        }
        Self::match_keyword_impl(s)
    }

    fn match_keyword_impl(s: &str) -> Self {
        match s {
            "of" => Of,
            "even" => Even,
            "odd" => Odd,
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
            Of => "of",
            Url => "url",
            LAngle => "<",
            LBrack => "[",
            LCurly => "{",
            LParen => "(",
            RAngle => ">",
            RBrack => "]",
            RCurly => "}",
            RParen => ")",
            Even => "even",
            Odd => "odd",
            N => "n",
            Amp => "&",
            Bang => "!",
            Caret => "^",
            Colon => ":",
            Colon2 => "::",
            At => "@",
            Comma => ",",
            Tilde => "~",
            Dollar => "$",
            Dot => ".",
            Eq => "=",
            Minus => "-",
            Percent => "%",
            Pipe => "|",
            Semicolon => ";",
            Pipe2 => "||",
            Str => "Str",
            Star => "*",
            Plus => "+",
            Slash => "/",
            Number => "Number",
            Question => "?",
            Backtick => "`",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
