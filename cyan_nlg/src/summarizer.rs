use rust_bert::pipelines::summarization::SummarizationModel;
use std::time::Instant;
use std::thread;

pub fn summarize(text: &str) -> String {
    let t = Instant::now();
    println!("Thread {:?} started: {:?}", thread::current().id(), t);
    let res = SummarizationModel::new(
        Default::default(),
    )
        .unwrap()
        .summarize([text])
        .join(" ");
    println!("Thread {:?} took: {:?}", thread::current().id(), t.elapsed());
    res
}
