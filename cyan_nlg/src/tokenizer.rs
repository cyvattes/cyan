use tokenizers::tokenizer::{Tokenizer};

pub fn tokenize(text: &str) -> String {
    Tokenizer::from_pretrained(
        "facebook/bart-large-cnn",
        None,
    )
        .unwrap()
        .encode(
        text,
        false
    )
        .unwrap()
        .get_tokens()
        .join(", ")
}
