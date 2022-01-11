use crate::utils::Config;
use rust_bert::pipelines::summarization::SummarizationModel;

pub(crate) fn from(text: &str, config: Config) -> String {
    let res = SummarizationModel::new(
        config.config()
    )
        .unwrap()
        .summarize([text])
        .join(" ");
    res
}
