use rust_bert::pipelines::summarization::SummarizationModel;

pub fn summarize(text: &str) -> String {
    SummarizationModel::new(
        Default::default(),
    )
        .unwrap()
        .summarize([text])
        .join(" ")
}
