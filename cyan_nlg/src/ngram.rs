use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct NGram(Vec<String>);

impl NGram {
    pub fn new() -> Vec<String> {
        vec![String::new()]
    }

    pub async fn from(text: String, n: usize) -> Vec<String> {
        let t: String = text
            .chars()
            .filter(|c| c.is_ascii() && !c.is_ascii_punctuation())
            .collect();
        build(&t, n).await
    }
}

async fn build(text: &str, n: usize) -> Vec<String> {
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
