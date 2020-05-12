use crate::parser::Parser;
use crate::value::Json;

pub mod parser;
pub mod token;
pub mod tokenizer;
pub mod value;

fn do_nothing() {}

pub fn parse(s: &str) -> Json {
    let mut parser = Parser::new(s);
    parser.product()
}

pub fn stringify<T>(_o: T) -> String
where
    T: Into<Json>,
{
    // TODO
    String::new()
}
