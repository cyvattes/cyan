mod samples;
mod summarizer;
// mod tokenizer;

use std::time::SystemTime;

pub fn run_short() -> Vec<String> {
    let t = SystemTime::now();
    let s = summarizer::summarize(samples::SHORT);
    match t.elapsed() {
        Ok(elapsed) => println!("Task took {}s\n", elapsed.as_secs()),
        Err(e) => println!("{}\n", e),
    };

    s
}

pub fn run() {
    // println!("Starting Short Tokenizer...");
    // let t = SystemTime::now();
    // tokenizer::tokenize(samples::SHORT);
    // match t.elapsed() {
    //     Ok(elapsed) => println!("Task took {}s\n", elapsed.as_secs()),
    //     Err(e) => println!("{}\n", e),
    // }

    println!("Starting Short Summarizer...");
    let t = SystemTime::now();
    summarizer::summarize(samples::SHORT);
    match t.elapsed() {
        Ok(elapsed) => println!("Task took {}s\n", elapsed.as_secs()),
        Err(e) => println!("{}\n", e),
    }

    // println!("Starting Long Tokenizer...");
    // let t = SystemTime::now();
    // tokenizer::tokenize(samples::LONG);
    // match t.elapsed() {
    //     Ok(elapsed) => println!("Task took {}s\n", elapsed.as_secs()),
    //     Err(e) => println!("{}\n", e),
    // }

    println!("Starting Long Summarizer...");
    let t = SystemTime::now();
    summarizer::summarize(samples::LONG);
    match t.elapsed() {
        Ok(elapsed) => println!("Task took {}s\n", elapsed.as_secs()),
        Err(e) => println!("{}\n", e),
    }
}
