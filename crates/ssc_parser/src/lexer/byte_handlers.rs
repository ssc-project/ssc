use memchr::memchr2;

use super::{Kind, Lexer};
use crate::diagnostics;

#[allow(clippy::unnecessary_safety_comment)]
/// Handle next byte of source.
///
/// SAFETY:
/// * Lexer must not be at end of file.
/// * `byte` must be next byte of source code, corresponding to current position
///   of `lexer.source`.
/// * Only `BYTE_HANDLERS` for ASCII characters may use the
///   `ascii_byte_handler!()` macro.
pub(super) unsafe fn handle_byte(byte: u8, lexer: &mut Lexer) -> Kind {
    BYTE_HANDLERS[byte as usize](lexer)
}

type ByteHandler = unsafe fn(&mut Lexer<'_>) -> Kind;

/// Lookup table mapping any incoming byte to a handler function defined below.
/// <https://github.com/ratel-rust/ratel-core/blob/master/ratel/src/lexer/mod.rs>
#[rustfmt::skip]
static BYTE_HANDLERS: [ByteHandler; 256] = [
//  0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F    //
    ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, SPS, LIN, ISP, ISP, LIN, ERR, ERR, // 0
    ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, // 1
    SPS, EXL, QOD, HAS, IDT, PRC, AMP, QOS, PNO, PNC, ATR, PLS, COM, MIN, PRD, SLH, // 2
    DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, COL, SEM, LSS, EQL, GTR, QST, // 3
    AT_, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 4
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, BTO, ESC, BTC, CRT, IDT, // 5
    TPL, L_A, IDT, L_C, L_D, L_E, IDT, IDT, L_H, L_I, IDT, L_K, IDT, IDT, IDT, IDT, // 6
    IDT, IDT, L_R, L_S, L_T, IDT, IDT, IDT, IDT, IDT, IDT, BEO, PIP, BEC, TLD, ERR, // 7
    UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, // 8
    UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, // 9
    UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, // A
    UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, // B
    UER, UER, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // C
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // D
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // E
    UNI, UNI, UNI, UNI, UNI, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, UER, // F
];

/// Macro for defining a byte handler.
///
/// Use `ascii_byte_handler!` macro for ASCII characters, which adds
/// optimizations for ASCII.
///
/// Handlers are defined as functions instead of closures, so they have names in
/// flame graphs.
///
/// ```
/// byte_handler!(UNI(lexer) {
///   lexer.unicode_char_handler()
/// });
/// ```
///
/// expands to:
///
/// ```
/// const UNI: ByteHandler = {
///   #[allow(non_snake_case)]
///   fn UNI(lexer: &mut Lexer) -> Kind {
///     lexer.unicode_char_handler()
///   }
///   UNI
/// };
/// ```
macro_rules! byte_handler {
    ($id:ident($lex:ident) $body:expr) => {
        const $id: ByteHandler = {
            #[allow(non_snake_case)]
            fn $id($lex: &mut Lexer) -> Kind {
                $body
            }
            $id
        };
    };
}

#[allow(clippy::unnecessary_safety_comment)]
/// Macro for defining byte handler for an ASCII character.
///
/// In addition to defining a `const` for the handler, it also asserts that
/// lexer is not at end of file, and that next char is ASCII.
/// Where the handler is for an ASCII character, these assertions are
/// self-evidently true.
///
/// These assertions produce no runtime code, but hint to the compiler that it
/// can assume that next char is ASCII, and it uses that information to optimize
/// the rest of the handler. e.g. `lexer.consume_char()` becomes just a single
/// assembler instruction. Without the assertions, the compiler is unable to
/// deduce the next char is ASCII, due to the indirection of the `BYTE_HANDLERS`
/// jump table.
///
/// These assertions are unchecked (i.e. won't panic) and will cause UB if
/// they're incorrect.
///
/// # SAFETY
/// Only use this macro to define byte handlers for ASCII characters.
///
/// ```
/// ascii_byte_handler!(SPS(lexer) {
///   lexer.consume_char();
///   Kind::WhiteSpace
/// });
/// ```
///
/// expands to:
///
/// ```
/// const SPS: ByteHandler = {
///   #[allow(non_snake_case)]
///   fn SPS(lexer: &mut Lexer) {
///     // SAFETY: This macro is only used for ASCII characters
///     unsafe {
///       use assert_unchecked::assert_unchecked;
///       let s = lexer.current.chars.as_str();
///       assert_unchecked!(!lexer.source.is_eof());
///       assert_unchecked!(lexer.source.peek_byte_unchecked() < 128);
///     }
///     {
///       lexer.consume_char();
///       Kind::WhiteSpace
///     }
///   }
///   SPS
/// };
/// ```
macro_rules! ascii_byte_handler {
    ($id:ident($lex:ident) $body:expr) => {
        byte_handler!($id($lex) {
            // SAFETY: This macro is only used for ASCII characters
            unsafe {
                use assert_unchecked::assert_unchecked;
                assert_unchecked!(!$lex.source.is_eof());
                assert_unchecked!($lex.source.peek_byte_unchecked() < 128);
            }
            $body
        });
    };
}

#[allow(clippy::unnecessary_safety_comment)]
/// Macro for defining byte handler for an ASCII character which is start of an
/// identifier (`a`-`z`, `A`-`Z`, `$` or `_`).
///
/// Macro calls `Lexer::identifier_name_handler` to get the text of the
/// identifier, minus its first character.
///
/// `Lexer::identifier_name_handler` is an unsafe function, but if byte being
/// consumed is ASCII, its requirements are met.
///
/// # SAFETY
/// Only use this macro to define byte handlers for ASCII characters.
///
/// ```
/// ascii_identifier_handler!(L_G(id_without_first_char) match id_without_first_char {
///   "et" => Kind::Get,
///   "lobal" => Kind::Global,
///   _ => Kind::Ident,
/// });
/// ```
///
/// expands to:
///
/// ```
/// const L_G: ByteHandler = {
///   #[allow(non_snake_case)]
///   fn L_G(lexer: &mut Lexer) -> Kind {
///     // SAFETY: This macro is only used for ASCII characters
///     let id_without_first_char = unsafe { lexer.identifier_name_handler() };
///     match id_without_first_char {
///       "et" => Kind::Get,
///       "lobal" => Kind::Global,
///       _ => Kind::Ident,
///     }
///   }
///   L_G
/// };
/// ```
macro_rules! ascii_identifier_handler {
    ($id:ident($str:ident) $body:expr) => {
        byte_handler!($id(lexer) {
            // SAFETY: This macro is only used for ASCII characters
            let $str = unsafe { lexer.identifier_name_handler() };
            $body
        });
    };
}

// `\0` `\1` etc
ascii_byte_handler!(ERR(lexer) {
    let c = lexer.consume_char();
    lexer.error(diagnostics::invalid_character(c, lexer.unterminated_range()));
    Kind::Undetermined
});

// <SPACE> <TAB> Normal Whitespace
ascii_byte_handler!(SPS(lexer) {
    lexer.consume_char();
    Kind::Skip
});

// <VT> <FF> Irregular Whitespace
ascii_byte_handler!(ISP(lexer) {
    lexer.consume_char();
    lexer.trivia_builder.add_irregular_whitespace(lexer.token.start, lexer.offset());
    Kind::Skip
});

// '\r' '\n'
ascii_byte_handler!(LIN(lexer) {
    lexer.consume_char();
    lexer.line_break_handler()
});

// !
ascii_byte_handler!(EXL(lexer) {
    lexer.consume_char();
    Kind::Bang
});

// "
ascii_byte_handler!(QOD(lexer) {
    // SAFETY: This function is only called for `"`
    unsafe { lexer.read_string_literal_double_quote() }
});

// '
ascii_byte_handler!(QOS(lexer) {
    // SAFETY: This function is only called for `'`
    unsafe { lexer.read_string_literal_single_quote() }
});

// #
ascii_byte_handler!(HAS(lexer) {
    lexer.consume_char();
    Kind::Hash
});

// `A..=Z`, `a..=z` (except special cases below), `_`, `$`
ascii_identifier_handler!(IDT(_id_without_first_char) {
    Kind::Ident
});

// %
ascii_byte_handler!(PRC(lexer) {
    lexer.consume_char();
    Kind::Percent
});

// &
ascii_byte_handler!(AMP(lexer) {
    lexer.consume_char();
    Kind::Amp
});

// (
ascii_byte_handler!(PNO(lexer) {
    lexer.consume_char();
    Kind::LParen
});

// )
ascii_byte_handler!(PNC(lexer) {
    lexer.consume_char();
    Kind::RParen
});

// *
ascii_byte_handler!(ATR(lexer) {
    lexer.consume_char();
    Kind::Star
});

// +
ascii_byte_handler!(PLS(lexer) {
    lexer.consume_char();
    Kind::Plus
});

// ,
ascii_byte_handler!(COM(lexer) {
    lexer.consume_char();
    Kind::Comma
});

// -
ascii_byte_handler!(MIN(lexer) {
    lexer.consume_char();
    Kind::Minus
});

// .
ascii_byte_handler!(PRD(lexer) {
    lexer.consume_char();
    Kind::Dot
});

// /
ascii_byte_handler!(SLH(lexer) {
    lexer.consume_char();
    Kind::Slash
});

// 0 to 9
ascii_byte_handler!(DIG(lexer) {
    lexer.consume_char();
    Kind::Number
});

// :
ascii_byte_handler!(COL(lexer) {
    lexer.consume_char();
    Kind::Colon
});

// ;
ascii_byte_handler!(SEM(lexer) {
    lexer.consume_char();
    Kind::Semicolon
});

// <
ascii_byte_handler!(LSS(lexer) {
    lexer.consume_char();
    if lexer.remaining().starts_with("!--") {
        lexer.consume_char();
        lexer.consume_char();
        lexer.consume_char();
        lexer.skip_comment()
    } else {
        Kind::LAngle
    }
});

// =
ascii_byte_handler!(EQL(lexer) {
    lexer.consume_char();
    Kind::Eq
});

// >
ascii_byte_handler!(GTR(lexer) {
    lexer.consume_char();
    // `>=` is re-lexed with [Lexer::next_jsx_child]
    Kind::RAngle
});

// ?
ascii_byte_handler!(QST(lexer) {
    lexer.consume_char();
    Kind::Question
});

// @
ascii_byte_handler!(AT_(lexer) {
    lexer.consume_char();
    Kind::At
});

// [
ascii_byte_handler!(BTO(lexer) {
    lexer.consume_char();
    Kind::LBrack
});

// \
ascii_byte_handler!(ESC(lexer) {
    lexer.identifier_backslash_handler()
});

// ]
ascii_byte_handler!(BTC(lexer) {
    lexer.consume_char();
    Kind::RBrack
});

// ^
ascii_byte_handler!(CRT(lexer) {
    lexer.consume_char();
    Kind::Caret
});

// `
ascii_byte_handler!(TPL(lexer) {
    lexer.consume_char();
    Kind::Backtick
});

// {
ascii_byte_handler!(BEO(lexer) {
    lexer.consume_char();
    Kind::LCurly
});

// |
ascii_byte_handler!(PIP(lexer) {
    lexer.consume_char();
    Kind::Pipe
});

// }
ascii_byte_handler!(BEC(lexer) {
    lexer.consume_char();
    Kind::RCurly
});

// ~
ascii_byte_handler!(TLD(lexer) {
    lexer.consume_char();
    Kind::Tilde
});

ascii_identifier_handler!(L_A(id_without_first_char) match id_without_first_char {
    "s" => Kind::As,
    "wait" => Kind::Await,
    _ => Kind::Ident,
});

ascii_identifier_handler!(L_C(id_without_first_char) match id_without_first_char {
    "atch" => Kind::Catch,
    "onst" => Kind::Const,
    _ => Kind::Ident,
});

ascii_identifier_handler!(L_D(id_without_first_char) match id_without_first_char {
    "ebug" => Kind::Debug,
    _ => Kind::Ident,
});

ascii_identifier_handler!(L_E(id_without_first_char) match id_without_first_char {
    "ach" => Kind::Each,
    "lse" => Kind::Else,
    _ => Kind::Ident,
});

ascii_identifier_handler!(L_H(id_without_first_char) match id_without_first_char {
    "tml" => Kind::Html,
    _ => Kind::Ident,
});

ascii_identifier_handler!(L_I(id_without_first_char) match id_without_first_char {
    "f" => Kind::If,
    _ => Kind::Ident,
});

ascii_identifier_handler!(L_K(id_without_first_char) match id_without_first_char {
    "ey" => Kind::Key,
    _ => Kind::Ident,
});

ascii_identifier_handler!(L_R(id_without_first_char) match id_without_first_char {
    "ender" => Kind::Render,
    _ => Kind::Ident,
});

ascii_identifier_handler!(L_S(id_without_first_char) match id_without_first_char {
    "cript" => Kind::Script,
    "nippet" => Kind::Snippet,
    "tyle" => Kind::Style,
    _ => Kind::Ident,
});

ascii_identifier_handler!(L_T(id_without_first_char) match id_without_first_char {
    "hen" => Kind::Then,
    _ => Kind::Ident,
});

// Non-ASCII characters.
// NB: Must not use `ascii_byte_handler!` macro, as this handler is for
// non-ASCII chars.
byte_handler!(UNI(lexer) {
    let len = memchr2(b'{', b'<', lexer.remaining().as_bytes());
    if let Some(len) = len {
        // SAFETY: `memchr2` guarantees `len` will be offset from
        // current position of a `{` or `<`
        // byte. So must be a valid UTF-8 boundary, and within
        // bounds of source.
        let end = unsafe { lexer.source.position().add(len) };
        lexer.source.set_position(end);
    } else {
        lexer.source.advance_to_end();
    }
    Kind::Text
});

// UTF-8 continuation bytes (0x80 - 0xBF) (i.e. middle of a multi-byte UTF-8
// sequence)
// + and byte values which are not legal in UTF-8 strings (0xC0, 0xC1, 0xF5 -
//   0xFF).
// `handle_byte()` should only be called with 1st byte of a valid UTF-8
// character, so something has gone wrong if we get here.
// https://datatracker.ietf.org/doc/html/rfc3629
// NB: Must not use `ascii_byte_handler!` macro, as this handler is for
// non-ASCII bytes.
byte_handler!(UER(_lexer) {
    unreachable!();
});
