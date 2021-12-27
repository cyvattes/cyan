use serde::{Serialize};
// use tokenizers::tokenizer::{Tokenizer};

#[derive(Serialize)]
pub struct NGram {
    n1: Vec<String>,
    n2: Vec<String>,
    n3: Vec<String>,
    n4: Vec<String>,
}

impl NGram {
    pub fn new() -> NGram {
        NGram {
            n1: vec![String::new()],
            n2: vec![String::new()],
            n3: vec![String::new()],
            n4: vec![String::new()],
        }
    }

    pub async fn from(text: String) -> NGram {
        NGram {
            n1: get_ngrams(&text, 1).await,
            n2: get_ngrams(&text, 2).await,
            n3: get_ngrams(&text, 3).await,
            n4: get_ngrams(&text, 4).await,
        }
    }
}

async fn get_ngrams(text: &str, n: i8) -> Vec<String> {
    // TODO: loop over input text, collect groups of n, vec!
    println!("{}, {}", n, text);
    vec![String::from(text)]
}

/* DEBUG TOKENIZER
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
 */
