use std::collections::HashMap;

use crate::do_nothing;
use crate::error::JsonError;
use crate::token::Token;
use crate::tokenizer::Tokenizer;
use crate::value::Json;
use crate::Result;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            tokenizer: Tokenizer::new(s),
        }
    }

    pub fn parse(&mut self) -> Result<Json> {
        let token = self.step()?;

        self.parse_from(token)
    }

    fn step(&mut self) -> Result<Token> {
        // TODO
        self.tokenizer.next().expect("Unexpected end of JSON!!!")
    }

    fn parse_array(&mut self) -> Result<Json> {
        let mut array = Vec::new();

        match self.step()? {
            Token::BracketOff => return Ok(array.into()),
            token => array.push(self.parse_from(token)?),
        }

        loop {
            match self.step()? {
                Token::Comma => array.push(self.parse()?),
                Token::BracketOff => break,
                token => return Err(JsonError::UnexpectedToken(token)),
            }
        }

        Ok(array.into())
    }

    fn parse_object(&mut self) -> Result<Json> {
        let mut object = HashMap::new();

        match self.step()? {
            Token::BraceOff => return Ok(object.into()),
            Token::String(key) => {
                match self.step()? {
                    Token::Colon => do_nothing(),
                    token => return Err(JsonError::UnexpectedToken(token)),
                }
                let value = self.parse()?;
                object.insert(key, value);
            }
            token => return Err(JsonError::UnexpectedToken(token)),
        }

        loop {
            match self.step()? {
                Token::Comma => {
                    let key = match self.step()? {
                        Token::String(key) => key,
                        token => return Err(JsonError::UnexpectedToken(token)),
                    };
                    match self.step()? {
                        Token::Colon => {}
                        token => return Err(JsonError::UnexpectedToken(token)),
                    }
                    let value = self.parse()?;
                    object.insert(key, value);
                }
                Token::BraceOff => break,
                token => return Err(JsonError::UnexpectedToken(token)),
            }
        }

        Ok(object.into())
    }

    fn parse_from(&mut self, token: Token) -> Result<Json> {
        match token {
            Token::Null => Ok(Json::Null),
            Token::String(s) => Ok(Json::String(s)),
            Token::Number(n) => Ok(Json::Number(n)),
            Token::Boolean(b) => Ok(Json::Boolean(b)),
            Token::BracketOn => self.parse_array(),
            Token::BraceOn => self.parse_object(),
            _ => Err(JsonError::UnexpectedToken(token)),
        }
    }
}
