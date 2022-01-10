use rust_bert::bart::{BartConfigResources, BartMergesResources, BartModelResources, BartVocabResources};
use rust_bert::gpt2::Gpt2MergesResources;
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::summarization::SummarizationConfig;
use rust_bert::prophetnet::{ProphetNetConfigResources, ProphetNetModelResources, ProphetNetVocabResources};
use rust_bert::resources::{RemoteResource, Resource::Remote};
use rust_bert::t5::{T5ConfigResources, T5ModelResources, T5VocabResources};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Config {
    BART,
    T5GN,
    PNET,
}

impl Config {
    pub(crate) fn config(&self) -> SummarizationConfig {
        match *self {
            Config::BART => SummarizationConfig::new(
                ModelType::Bart,
                Remote(RemoteResource::from_pretrained(
                    BartModelResources::BART_CNN
                )),
                Remote(RemoteResource::from_pretrained(
                    BartConfigResources::BART_CNN
                )),
                Remote(RemoteResource::from_pretrained(
                    BartVocabResources::BART_CNN
                )),
                Remote(RemoteResource::from_pretrained(
                    BartMergesResources::BART_CNN
                )),
            ),
            Config::T5GN => SummarizationConfig::new(
                ModelType::T5,
                Remote(RemoteResource::from_pretrained(
                    T5ModelResources::T5_BASE
                )),
                Remote(RemoteResource::from_pretrained(
                    T5ConfigResources::T5_BASE
                )),
                Remote(RemoteResource::from_pretrained(
                    T5VocabResources::T5_BASE
                )),
                Remote(RemoteResource::from_pretrained(
                    Gpt2MergesResources::GPT2_LARGE
                )),
            ),
            Config::PNET => SummarizationConfig::new(
                ModelType::ProphetNet,
                Remote(RemoteResource::from_pretrained(
                    ProphetNetModelResources::PROPHETNET_LARGE_CNN_DM
                )),
                Remote(RemoteResource::from_pretrained(
                    ProphetNetConfigResources::PROPHETNET_LARGE_CNN_DM
                )),
                Remote(RemoteResource::from_pretrained(
                    ProphetNetVocabResources::PROPHETNET_LARGE_CNN_DM
                )),
                Remote(RemoteResource::from_pretrained(
                    BartMergesResources::BART_CNN
                )),
            ),
        }
    }
}

pub fn map_to_sorted_vec(map: HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut sorted: Vec<(String, u32)> = map.into_iter().collect();
    sorted.sort_by(|a, b| match a.1.cmp(&b.1).reverse() {
        Ordering::Equal => a.0.cmp(&b.0),
        v => v,
    });
    sorted
}