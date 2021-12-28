use tokenizers::tokenizer::{Tokenizer};

// DEBUG TOKENIZER
pub fn _tokenize(text: &str) -> String {
    Tokenizer::from_pretrained(
        "facebook/bart-large-cnn",
        None,
    )
        .unwrap()
        .encode(
        text,
        true,
    )
        .unwrap()
        .get_tokens()
        .join(", ")
}
