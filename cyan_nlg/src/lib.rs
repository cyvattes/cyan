pub mod samples;
mod summarizer;
mod tokenizer;

use tokio;

pub async fn summarize(text: &'static str) -> String {
    run(summarizer::summarize, text).await
}

pub async fn tokenize(text: &'static str) -> String {
    run(tokenizer::tokenize, text).await
}

async fn run(f: fn(&str) -> String, arg: &'static str) -> String {
    tokio::task::spawn_blocking(move || {
        f(arg)
    })
        .await
        .expect("Thread panicked")
}
