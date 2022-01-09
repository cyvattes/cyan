use crate::utils::map_to_sorted_vec;
use rust_bert::pipelines::pos_tagging::{POSModel, POSTag};
use std::collections::HashMap;

pub(crate) fn from(text: &str) -> Vec<(String, u32)> {
    let tags: Vec<POSTag> = POSModel::new(
        Default::default(),
    )
        .unwrap()
        .predict([text])
        .into_iter()
        .flatten()
        .collect();

    let mut map: HashMap<String, u32> = HashMap::new();
    for tag in tags.iter() {
        *map.entry(tag.label.to_string()).or_insert(0) += 1;
    };

    let res = map_to_sorted_vec(map);
    println!("{}: {:?}", res.len(), res);
    res
}