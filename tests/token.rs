use json::token::Tokenizer;

#[cfg(test)]
pub mod tests {
    use json::token::Tokenizer;

    #[test]
    pub fn test_tokenizer_creatable() {
        let tokenizer = Tokenizer::new("{}");
    }
}
