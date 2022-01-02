mod bleu;
pub mod ngram;
mod summarizer;
mod tokenizer;

use std::future::Future;
// use futures::future::try_join_all;
use ngram::NGram;
use tokio::{
    self,
    // try_join,
    // task::JoinHandle,
    // time::Instant,
};

pub async fn summarize(text: &str) -> String {
    text.to_string()

    // TODO: Summarizer is sharing resources, forcing
    //  threads to be run concurrently, but not
    //  in parallel. Need to see if we can fix this.
    // let t = text.to_string();
    // tokio::task::spawn_blocking(move || {
    //     summarizer::summarize(&t)
    // })
    //     .await
    //     .expect("Thread panicked")
}

pub async fn tokenize(text: &str, n: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let t = text.to_string();
    let r = tokio::task::spawn_blocking(move || {
        NGram::from(t, n)
    })
        .await
        .expect("Thread panicked")
        .await;
    Ok(r)
}

pub fn bleu(src: &Vec<String>, abs: &Vec<String>) -> String {
    bleu::calculate(src, abs)
}
