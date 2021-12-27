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
        let t: String = text
            .chars()
            .filter(|c| c.is_ascii() && !c.is_ascii_punctuation())
            .collect();

        NGram {
            n1: get_ngrams(&t, 1).await,
            n2: get_ngrams(&t, 2).await,
            n3: get_ngrams(&t, 3).await,
            n4: get_ngrams(&t, 4).await,
        }
    }
}

async fn get_ngrams(text: &str, n: usize) -> Vec<String> {
    let t: Vec<_> = text
        .split_ascii_whitespace()
        .collect();

    if t.len() < n {
        return vec![];
    }

    let mut ngram = Vec::new();
    for i in 0..=t.len()-n {
        let mut v = Vec::new();
        for j in 0..n {
            v.push(t[i+j]);
        }

        ngram.push(v.join(" "));
    }
    ngram
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
