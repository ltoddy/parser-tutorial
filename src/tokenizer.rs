use std::iter::Peekable;
use std::str::Chars;

use crate::token::Token;

pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            source: s.chars().peekable(),
        }
    }

    // 对于null, true, false这样的符号, 只需要一步一步的向后便利判断就可以了.
    fn read_symbol(&mut self, first: char) -> String {
        let mut symbol = first.to_string();

        while let Some(&ch) = self.source.peek() {
            match ch {
                'a'..='z' => {
                    symbol.push(ch);
                    self.source.next();
                }
                _ => break, // 遇到非英文小写字母，判定它结束
            }
        }

        symbol
    }

    // "xxx 12.345 xxx"
    //      ^^^^^^
    //        |
    //     这里是数字
    // 可能带有小数点,也可能不带有小数点,
    // 那么,我们可以设立一个游标cursor,去一步一步的前进(cursor++),
    // 如果如果是字符数字,那么保存这个数字,并且向下一步前进.
    // 当遇到小数点的时候,判断这个小数点是否是第一次出现, 如果是第一次出现,
    // 那没什么问题,如果是第二次出现及以上,那么说明这个数字字符串是无效非法的.
    fn read_number(&mut self, first: char) -> f64 {
        let mut value = first.to_string();
        let mut point = false;

        while let Some(&ch) = self.source.peek() {
            match ch {
                '0'..='9' | '-' => {
                    value.push(ch);
                    self.source.next();
                }
                '.' => {
                    if !point {
                        point = true;
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

    // r#" "this is a string" "
    //     ^^^^^^^^^^^^^^^^^^
    // 对于字符串来说,以双引号开头,以双引号结尾,同时,字符串中可能会有'\'反斜杠开头的转义字符.
    //
    fn read_string(&mut self, first: char) -> String {
        let mut value = String::new();
        let mut escape = false;

        while let Some(ch) = self.source.next() {
            if ch == first && !escape {
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
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        'lex: while let Some(ch) = self.source.next() {
            return Some(match ch {
                ',' => Token::Comma,
                ':' => Token::Colon,
                '[' => Token::BracketOn,
                ']' => Token::BracketOff,
                '{' => Token::BraceOn,
                '}' => Token::BraceOff,
                '"' => Token::String(self.read_string(ch)),
                '0'..='9' | '-' => Token::Number(self.read_number(ch)),
                'a'..='z' => {
                    let label = self.read_symbol(ch);
                    match label.as_ref() {
                        "true" => Token::Boolean(true),
                        "false" => Token::Boolean(false),
                        "null" => Token::Null,
                        _ => panic!("Invalid label: {}", label),
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
