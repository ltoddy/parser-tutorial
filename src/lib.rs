pub mod generate;
pub mod parser;
pub mod token;
pub mod tokenizer;
pub mod value;

use crate::generate::CodeGenerator;
use crate::parser::Parser;
use crate::value::Json;

fn do_nothing() {}

pub fn parse(s: &str) -> Json {
    let mut parser = Parser::new(s);
    parser.product()
}

pub fn stringify<T>(o: T) -> String
where
    T: Into<Json>,
{
    let mut gen = CodeGenerator::new();
    gen.gather(&o.into());
    gen.product()
}
