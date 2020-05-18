use std::fmt::{Debug, Display, Formatter, Result};

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

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, f)
    }
}
