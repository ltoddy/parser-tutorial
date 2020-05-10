use std::iter::Peekable;
use std::str::Chars;

/// 对于Json的token有：
/// `,`, `:`, `{`, `}`, `[`, `]`, `String`, `Number`, `Boolean`, `Null`
#[derive(Debug)]
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

pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            source: s.chars().peekable(),
        }
    }

    fn read_label(&mut self, first: char) -> String {
        let mut label = first.to_string();

        while let Some(&ch) = self.source.peek() {
            match ch {
                'a'..='z' => {
                    label.push(ch);
                    self.source.next();
                }
                _ => break,
            }
        }

        label
    }

    fn read_string(&mut self, first: char) -> String {
        let mut value = String::new();
        let mut escape = false;

        while let Some(ch) = self.source.next() {
            if ch == first && escape == false {
                return value;
            }
            match ch {
                '\\' => {
                    if escape {
                        escape = false;
                        value.push(ch);
                    } else {
                        escape = true;
                    }
                }
                _ => {
                    value.push(ch);
                    escape = false;
                }
            }
        }

        value
    }

    fn read_number(&mut self, first: char) -> f64 {
        let mut value = first.to_string();
        let mut period = false;

        while let Some(&ch) = self.source.peek() {
            match ch {
                '0'..='9' => {
                    value.push(ch);
                    self.source.next();
                }
                '.' => {
                    if !period {
                        period = true;
                        value.push(ch);
                        self.source.next();
                    } else {
                        return value.parse::<f64>().unwrap();
                    }
                }
                _ => return value.parse::<f64>().unwrap(),
            }
        }

        value.parse::<f64>().unwrap()
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        'lex: while let Some(ch) = self.source.next() {
            return Some(match ch {
                '.' => Token::Comma,
                ':' => Token::Colon,
                '[' => Token::BraceOn,
                ']' => Token::BraceOff,
                '{' => Token::BracketOn,
                '}' => Token::BracketOff,
                '"' => Token::String(self.read_string(ch)),
                '0'..='9' => Token::Number(self.read_number(ch)),
                'a'..='z' => {
                    let label = self.read_label(ch);
                    match label.as_ref() {
                        "true" => Token::Boolean(true),
                        "false" => Token::Boolean(false),
                        "null" => Token::Null,
                        _ => panic!("Invalid symbol: {}", label),
                    }
                }
                _ => {
                    if ch.is_whitespace() {
                        continue 'lex;
                    } else {
                        panic!("Invalid character: {}", ch);
                    }
                }
            });
        }

        None
    }
}
