use std::fmt::{Display, Formatter, Result};

use crate::token::Token;

#[derive(Debug)]
pub enum JsonError {
    UnexpectedToken(Token),
    InvalidCharacter(char),
    InvalidLabel(String),
}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            JsonError::UnexpectedToken(token) => write!(f, "unexpected token: {}", token),
            JsonError::InvalidCharacter(ch) => write!(f, "invalid character: {}", ch),
            JsonError::InvalidLabel(s) => write!(f, "Invalid label: {}", s),
        }
    }
}

impl std::error::Error for JsonError {}
