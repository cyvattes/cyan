use std::cmp::Ordering;
use std::collections::HashMap;

pub fn map_to_sorted_vec(map: HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut sorted: Vec<(String, u32)> = map.into_iter().collect();
    sorted.sort_by(|a, b| match a.1.cmp(&b.1).reverse() {
        Ordering::Equal => a.0.cmp(&b.0),
        v => v,
    });
    sorted
}