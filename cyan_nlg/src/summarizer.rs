use rust_bert::pipelines::summarization::SummarizationModel;

pub fn summarize(text: &str) {
    let s = SummarizationModel::new(
        Default::default(),
    ).unwrap();

    println!("{:?}", s.summarize([text]));
}
