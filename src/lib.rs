pub mod code_generator;
pub mod implement;
pub mod parser;
pub mod token;
pub mod tokenizer;
pub mod value;

use crate::code_generator::CodeGenerator;
use crate::parser::Parser;
use crate::value::Json;

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
