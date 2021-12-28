mod bleu;
pub mod ngram;
mod summarizer;
mod tokenizer;

use ngram::NGram;
use tokio;

pub async fn summarize(text: &str) -> String {
    let t = text.to_string();
    tokio::task::spawn_blocking(move || {
        summarizer::summarize(&t)
    })
        .await
        .expect("Thread panicked")
}

pub async fn tokenize(text: &str, n: usize) -> Vec<String> {
    let t = text.to_string();
    tokio::task::spawn_blocking(move || {
        NGram::from(t, n)
    })
        .await
        .expect("Thread panicked")
        .await
}

pub fn bleu(src: &Vec<String>, abs: &Vec<String>) -> String {
    bleu::calculate(src, abs)
}
