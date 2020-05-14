use std::collections::HashMap;

use crate::do_nothing;
use crate::token::Token;
use crate::tokenizer::Tokenizer;
use crate::value::Json;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            tokenizer: Tokenizer::new(s),
        }
    }

    fn step(&mut self) -> Token {
        self.tokenizer.next().expect("Unexpected end of JSON!!!")
    }

    fn parse_array(&mut self) -> Json {
        let mut array = Vec::new();

        match self.step() {
            Token::BracketOff => return array.into(),
            token => array.push(self.parse_from(token)),
        }

        loop {
            match self.step() {
                Token::Comma => array.push(self.parse()),
                Token::BracketOff => break,
                token => panic!("Unexpected token {:?}", token),
            }
        }

        array.into()
    }

    fn parse_object(&mut self) -> Json {
        let mut object = HashMap::new();

        match self.step() {
            Token::BraceOff => return object.into(),
            Token::String(key) => {
                match self.step() {
                    Token::Colon => do_nothing(),
                    token => panic!("Unexpected token {:?}", token),
                }
                let value = self.parse();
                object.insert(key, value);
            }
            token => panic!("Unexpected token {:?}", token),
        }

        loop {
            match self.step() {
                Token::Comma => {
                    let key = match self.step() {
                        Token::String(key) => key,
                        token => panic!("Unexpected token {:?}", token),
                    };
                    match self.step() {
                        Token::Colon => {}
                        token => panic!("Unexpected token {:?}", token),
                    }
                    let value = self.parse();
                    object.insert(key, value);
                }
                Token::BraceOff => break,
                token => panic!("Unexpected token {:?}", token),
            }
        }

        object.into()
    }

    fn parse_from(&mut self, token: Token) -> Json {
        match token {
            Token::Null => Json::Null,
            Token::String(v) => Json::String(v),
            Token::Number(v) => Json::Number(v),
            Token::Boolean(v) => Json::Boolean(v),
            Token::BracketOn => self.parse_array(),
            Token::BraceOn => self.parse_object(),
            _ => panic!("Unexpected token: {:?}", token),
        }
    }

    pub fn parse(&mut self) -> Json {
        let token = self.step();

        self.parse_from(token)
    }
}
