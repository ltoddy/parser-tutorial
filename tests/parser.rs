#[cfg(test)]
pub mod parser_tests {
    use json::parser::Parser;

    #[test]
    pub fn test_parser_creatable() {
        let _parser = Parser::new("");
    }
}
