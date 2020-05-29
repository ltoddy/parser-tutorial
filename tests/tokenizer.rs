#[cfg(test)]
pub mod tokenizer_tests {
    use json::token::Token;
    use json::tokenizer::Tokenizer;

    #[test]
    pub fn test_tokenizer_creatable() {
        let _tokenizer = Tokenizer::new("{}");
    }

    #[test]
    pub fn should_parse_brackets() {
        let mut tokenizer = Tokenizer::new("{}");

        assert_eq!(tokenizer.next(), Some(Token::BraceOn));
        assert_eq!(tokenizer.next(), Some(Token::BraceOff));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    pub fn should_parse_braces() {
        let mut tokenizer = Tokenizer::new("[]");

        assert_eq!(tokenizer.next(), Some(Token::BracketOn));
        assert_eq!(tokenizer.next(), Some(Token::BracketOff));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    pub fn should_parse_null() {
        let mut tokenizer = Tokenizer::new("null");

        assert_eq!(tokenizer.next(), Some(Token::Null));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    pub fn should_parse_boolean_value() {
        let mut tokenizer = Tokenizer::new("true");
        assert_eq!(tokenizer.next(), Some(Token::Boolean(true)));
        assert_eq!(tokenizer.next(), None);

        let mut tokenizer = Tokenizer::new("false");
        assert_eq!(tokenizer.next(), Some(Token::Boolean(false)));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    pub fn should_parse_number_value() {
        let mut tokenizer = Tokenizer::new("123");
        assert_eq!(tokenizer.next(), Some(Token::Number(123.)));

        let mut tokenizer = Tokenizer::new("1.23");
        assert_eq!(tokenizer.next(), Some(Token::Number(1.23)));

        let mut tokenizer = Tokenizer::new("-1.23");
        assert_eq!(tokenizer.next(), Some(Token::Number(-1.23)));
    }

    #[test]
    pub fn should_parse_object() {
        let mut tokenizer = Tokenizer::new(r#"{ "key": "value" }"#);

        assert_eq!(tokenizer.next(), Some(Token::BraceOn));
        assert_eq!(tokenizer.next(), Some(Token::String("key".to_owned())));
        assert_eq!(tokenizer.next(), Some(Token::Colon));
        assert_eq!(tokenizer.next(), Some(Token::String("value".to_owned())));
        assert_eq!(tokenizer.next(), Some(Token::BraceOff));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    pub fn should_parse_array() {
        let mut tokenizer = Tokenizer::new(r#"{ "key": [1, 2, "v1"] }"#);

        assert_eq!(tokenizer.next(), Some(Token::BraceOn));
        assert_eq!(tokenizer.next(), Some(Token::String("key".to_owned())));
        assert_eq!(tokenizer.next(), Some(Token::Colon));
        assert_eq!(tokenizer.next(), Some(Token::BracketOn));
        assert_eq!(tokenizer.next(), Some(Token::Number(1.)));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), Some(Token::Number(2.)));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), Some(Token::String("v1".to_owned())));
        assert_eq!(tokenizer.next(), Some(Token::BracketOff));
        assert_eq!(tokenizer.next(), Some(Token::BraceOff));
        assert_eq!(tokenizer.next(), None);
    }
}
