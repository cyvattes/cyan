use rust_bert::pipelines::summarization::SummarizationModel;

pub fn summarize(text: &str) -> Vec<String> {
    let s = SummarizationModel::new(
        Default::default(),
    ).unwrap();

    s.summarize([text])
}
