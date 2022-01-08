use std::cmp::Ordering;
use std::collections::HashMap;

pub fn get_top(text: Vec<String>, n: usize) -> Vec<(String, usize)> {
    let mut map: HashMap<String, usize> = HashMap::new();
    for n_gram in text.to_owned().iter() {
        *map.entry(n_gram.to_string()).or_insert(0) += 1;
    };

    let mut sorted: Vec<(String, usize)> = map.into_iter().collect();
    sorted.sort_by(|a, b| match a.1.cmp(&b.1).reverse() {
        Ordering::Equal => a.0.cmp(&b.0),
        v => v,
    });
    sorted.resize_with(n, || { (String::new(), 0 as usize) });
    sorted
}