mod summarizer;
pub mod tokenizer;

use tokenizer::NGram;
use tokio;

pub async fn summarize(text: &str) -> String {
    let t = text.to_string();
    tokio::task::spawn_blocking(move || {
        summarizer::summarize(&t)
    })
        .await
        .expect("Thread panicked")
}

pub async fn tokenize(text: &str) -> NGram {
    let t = text.to_string();
    tokio::task::spawn_blocking(move || {
        NGram::from(t)
    })
        .await
        .expect("Thread panicked")
        .await
}
