mod bleu;
pub mod ngram;
mod summarizer;
mod tokenizer;

use futures::future::try_join_all;
use ngram::NGram;
use tokio::{
    self,
    // try_join,
    task::JoinHandle,
    time::Instant,
};

fn spawn(s: &str) -> JoinHandle<String> {
    let cp = s.to_string();
    tokio::task::spawn_blocking(move || {
        summarizer::summarize(&cp)
    })
}

pub async fn summarize(text: &str) -> String {
    // TODO: Summarizer is sharing resources, forcing
    //  threads to be run concurrently, but not
    //  in parallel. Need to see if we can fix this.
    // const R: usize = 3;
    // let mut handles = Vec::with_capacity(R);
    //
    // let start = Instant::now();
    // for _ in 0..R {
    //     handles.push(spawn(text))
    // }
    // let v = match try_join_all(handles).await {
    //     Ok(val) => val,
    //     Err(_) => vec![
    //         String::new(),
    //         String::new(),
    //         String::new(),
    //     ],
    // };
    // println!("{:?}\n{:?}\n{:?}\n{:?}", start.elapsed(), v[0], v[1], v[2]);
    // v[0].to_string()

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
