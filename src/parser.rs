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

    fn array(&mut self) -> Json {
        Json::Null
    }

    fn object(&mut self) -> Json {
        Json::Null
    }

    pub fn product(&mut self) -> Json {
        let mut token = self.step();

        match token {
            Token::Null => Json::Null,
            Token::String(v) => Json::String(v),
            Token::Number(v) => Json::Number(v),
            Token::Boolean(v) => Json::Boolean(v),
            Token::BraceOn => self.array(),
            Token::BracketOn => self.array(),
            _ => panic!("Unexpected token: {:?}", token),
        }
    }
}
