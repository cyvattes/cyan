mod bleu;
mod ngram;
mod summary;
mod token;
mod tokenizer;
pub mod utils;

use std::error::Error;
use tokio::task::spawn_blocking;
use utils::Config;

type StrHandle = Result<String, Box<dyn Error>>;
type VecHandle = Result<Vec<String>, Box<dyn Error>>;
type TupHandle = Result<Vec<(String, u32)>, Box<dyn Error>>;

pub fn strip(text: &str) -> String {
    let t: String = text
        .chars()
        .filter(|c| c.is_ascii() && !c.is_ascii_punctuation())
        .collect();
    t
}

pub async fn summarize(text: &str, config: Config) -> StrHandle {
    let t = text.to_string();
    let r = spawn_blocking(move || {
        summary::from(&t, config)
    })
        .await
        .expect("Thread panicked");
    Ok(r)
}

pub async fn tokenize(text: &str) -> TupHandle {
    let t = text.to_string();
    let r =  tokio::task::spawn_blocking(move || {
        token::from(&t)
    })
        .await
        .expect("Thread panicked");
    Ok(r)
}

pub async fn ngramize(text: &str, n: usize) -> VecHandle {
    let t = text.to_string();
    let r = tokio::task::spawn_blocking(move || {
        ngram::from(&t, n)
    })
        .await
        .expect("Thread panicked");
    Ok(r)
}

pub fn bleu(src: &Vec<String>, abs: &Vec<String>) -> String {
    bleu::calculate(src, abs)
}
