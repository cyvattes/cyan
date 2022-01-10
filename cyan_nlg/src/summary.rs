use crate::utils::Config;
use rust_bert::pipelines::summarization::SummarizationModel;
use std::time::Instant;

pub(crate) fn from(text: &str, config: Config) -> String {
    let t = Instant::now();
    println!("Thread {:?} started", config);
    let res = SummarizationModel::new(
        config.config()
    )
        .unwrap()
        .summarize([text])
        .join(" ");
    println!("Thread {:?} finished in {:?}", config, t.elapsed());
    res
}
