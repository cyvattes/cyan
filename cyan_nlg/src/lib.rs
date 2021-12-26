pub mod n_gram;
mod summarizer;
mod tokenizer;

use tokio;

pub async fn summarize(text: &str) -> String {
    let t = text.to_string();
    tokio::task::spawn_blocking(move || {
        summarizer::summarize(&t)
    })
        .await
        .expect("Thread panicked")
}
