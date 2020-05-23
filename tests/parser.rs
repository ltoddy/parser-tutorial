#[cfg(test)]
pub mod parser_tests {
    use json::hash_map;
    use json::parser::Parser;
    use json::value::Json;

    #[test]
    pub fn test_parser_creatable() {
        let _parser = Parser::new("");
    }

    #[test]
    pub fn should_parse_null_type() {
        let mut parser = Parser::new("null");
        let json = parser.parse();

        assert_eq!(json, Json::Null);
    }

    #[test]
    pub fn should_parse_bool_type() {
        let mut parser = Parser::new("false");
        let json = parser.parse();
        assert_eq!(json, Json::Boolean(false));

        let mut parser = Parser::new("true");
        let json = parser.parse();
        assert_eq!(json, Json::Boolean(true));
    }

    #[test]
    pub fn should_parse_number_type() {
        let mut parser = Parser::new("1.23");
        let json = parser.parse();
        assert_eq!(json, Json::Number(1.23));
    }

    #[test]
    pub fn should_parse_string_type() {
        let mut parser = Parser::new(r#""Hello world""#);
        let json = parser.parse();
        assert_eq!(json, Json::String(String::from("Hello world")));
    }

    #[test]
    pub fn should_parse_array_type() {
        let mut parser = Parser::new(r#"[1, 2, 3, 4]"#);
        let json = parser.parse();
        assert_eq!(
            json,
            Json::Array(vec![
                Json::Number(1.),
                Json::Number(2.),
                Json::Number(3.),
                Json::Number(4.)
            ])
        )
    }

    #[test]
    pub fn should_parse_object_type() {
        let mut parser = Parser::new(r#"{ "k1": "v1" }"#);
        let json = parser.parse();
        assert_eq!(
            json,
            Json::Object(hash_map! {
                String::from("k1") => Json::String(String::from("v1"))
            })
        );
    }

    #[test]
    pub fn should_parse_complex_object() {
        let mut parser = Parser::new(
            r#"
            {
              "id": 262804062,
              "full_name": "ltoddy/parser-tutorial",
              "private": false,
              "owner": {
                "login": "ltoddy",
                "id": 20920763
              }
            }
        "#,
        );
        let json = parser.parse();

        assert_eq!(
            json,
            Json::Object(hash_map! {
                String::from("id") => Json::Number(262804062.),
                String::from("full_name") => Json::String(String::from("ltoddy/parser-tutorial")),
                String::from("private") => Json::Boolean(false),
                String::from("owner") => Json::Object(hash_map!{
                    String::from("login") => Json::String(String::from("ltoddy")),
                    String::from("id") => Json::Number(20920763.),
                }),
            })
        );
    }
}
