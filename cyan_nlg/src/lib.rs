pub mod samples;
mod summarizer;
mod tokenizer;

use std::time;
use tokio;

pub async fn summarize(text: &str) -> String {
    // tokio::task::spawn_blocking(move || {
    //     let t = time::Instant::now();
    //     let resp = summarizer::summarize(text);
    //     println!("{} :: {:?}", text.len(), t.elapsed());
    //     resp
    // })
    //     .await
    //     .expect("Thread panicked")

    run(summarizer::summarize, text).await
}

// pub async fn tokenize(text: &str) -> String {
//     run(tokenizer::tokenize, text).await
// }

async fn run(f: fn(&str) -> String, arg: &str) -> String {
    let a = arg.to_string();
    tokio::task::spawn_blocking(move || {
        let t = time::Instant::now();
        let r = f(&a);
        println!("{} :: {:?}", a.len(), t.elapsed());
        r
    })
        .await
        .expect("Thread panicked")
}
