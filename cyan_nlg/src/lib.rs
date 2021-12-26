mod summarizer;
mod tokenizer;

use tokio;

pub async fn summarize(text: &str) -> String {
    run(summarizer::summarize, text).await
}

pub async fn tokenize(text: &str) -> String {
    run(tokenizer::tokenize, text).await
}

// async fn run2(f: fn(&str) -> &[String], arg: &str) -> &[String] {
//     let a = arg.to_string();
//     tokio::task::spawn_blocking(move || {
//         f(&a)
//     })
//         .await
//         .expect("Thread panicked")
// }

async fn run(f: fn(&str) -> String, arg: &str) -> String {
    let a = arg.to_string();
    tokio::task::spawn_blocking(move || {
        f(&a)
    })
        .await
        .expect("Thread panicked")
}
