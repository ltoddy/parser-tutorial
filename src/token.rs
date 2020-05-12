use std::iter::Peekable;
use std::str::Chars;

/// 对于Json的token有：
/// `,`, `:`, `{`, `}`, `[`, `]`, `String`, `Number`, `Boolean`, `Null`
#[derive(Debug, PartialEq)]
pub enum Token {
    Comma,
    Colon,
    BracketOn,
    BracketOff,
    BraceOn,
    BraceOff,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}
