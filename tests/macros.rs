#[cfg(test)]
pub mod hash_map_tests {
    use std::collections::HashMap;

    use json::hash_map;

    #[test]
    fn test_hash_map_macro_without_params() {
        let expected: HashMap<i32, i32> = HashMap::with_capacity(16);
        let product: HashMap<i32, i32> = hash_map!();
        assert_eq!(expected, product);
    }

    #[test]
    fn test_hash_map_macro_with_one_param() {
        let mut expected = HashMap::with_capacity(16);
        expected.insert("hello", "world");
        let product = hash_map!("hello" => "world");
        assert_eq!(expected, product);
    }

    #[test]
    fn test_hash_map_macro_with_capacity() {
        let mut expected = HashMap::with_capacity(20);
        expected.insert("hello", "world");
        let product = hash_map!("hello" => "world"; 20);
        assert_eq!(expected, product);
    }

    #[test]
    fn test_hash_map_macro_with_any_params() {
        let mut expected = HashMap::with_capacity(16);
        expected.insert("hello", "world");
        expected.insert("world", "hello");
        let product = hash_map! {
            "hello" => "world",
            "world" => "hello",
        };
        assert_eq!(expected, product);
    }

    #[test]
    fn test_hash_map_macro_with_any_params_and_without_trailing_comma() {
        let mut expected = HashMap::with_capacity(16);
        expected.insert("hello", "world");
        expected.insert("world", "hello");
        let product = hash_map! {
            "hello" => "world",
            "world" => "hello"
        };
        assert_eq!(expected, product);
    }
}
