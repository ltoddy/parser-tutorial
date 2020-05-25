#[cfg(test)]
pub mod code_generator_tests {
    use json::code_generator::CodeGenerator;
    use json::hash_map;
    use json::value::Json;

    #[test]
    pub fn test_code_generator_creatable() {
        let _gen = CodeGenerator::new();
    }

    #[test]
    pub fn should_generate_null() {
        let mut gen = CodeGenerator::new();
        gen.gather(&Json::Null);
        assert_eq!("null", gen.product())
    }

    #[test]
    pub fn should_generate_bool() {
        let mut gen = CodeGenerator::new();
        gen.gather(&Json::Boolean(true));
        assert_eq!("true", gen.product());

        let mut gen = CodeGenerator::new();
        gen.gather(&Json::Boolean(false));
        assert_eq!("false", gen.product());
    }

    #[test]
    pub fn should_generate_number() {
        let mut gen = CodeGenerator::new();
        gen.gather(&Json::Number(1.23));
        assert_eq!("1.23", gen.product());
    }

    #[test]
    pub fn should_generate_string() {
        let mut gen = CodeGenerator::new();
        gen.gather(&Json::String(String::from("Hello world")));
        assert_eq!("\"Hello world\"", gen.product());
    }

    #[test]
    pub fn should_generate_array() {
        let mut gen = CodeGenerator::new();
        gen.gather(&Json::Array(vec![
            Json::Number(1.),
            Json::Number(2.),
            Json::Number(3.),
            Json::Number(4.),
        ]));
        assert_eq!("[1,2,3,4]", gen.product());
    }

    #[test]
    pub fn should_generate_object() {
        let mut gen = CodeGenerator::new();
        gen.gather(&Json::Object(hash_map! {
            String::from("k1") => Json::String(String::from("v1"))
        }));
        assert_eq!(r#"{"k1":"v1"}"#, gen.product());
    }
}
