pub mod code_generator;
pub mod error;
pub mod implement;
pub mod macros;
pub mod parser;
pub mod token;
pub mod tokenizer;
pub mod value;

use crate::code_generator::CodeGenerator;
use crate::error::JsonError;
use crate::parser::Parser;
use crate::value::Json;

pub type Result<T> = std::result::Result<T, JsonError>;

fn do_nothing() {}

pub fn parse(s: &str) -> Json {
    let mut parser = Parser::new(s);
    parser.parse()
}

pub fn stringify<T>(o: T) -> String
where
    T: Into<Json>,
{
    let mut gen = CodeGenerator::new();
    gen.gather(&o.into());
    gen.product()
}
