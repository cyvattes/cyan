use std::collections::HashMap;

pub(crate) fn calculate(src: &Vec<String>, abs: &Vec<String>) -> String {
    let mut m = 0;          // values in ABS found in SRC, capped at M(max)
    let w = abs.len();  // Total values found in SRC

    // Loop over SRC, capture M(max) per value
    let mut src_map: HashMap<String,usize> = HashMap::new();
    for ngram in src {
        *src_map.entry(ngram.to_string()).or_insert(0) += 1;
    }

    // Loop over ABS, add to M per matched value
    for ngram in abs {
        match src_map.get_mut(ngram) {
            Some(x) if *x > 0 => {
                *x -= 1;
                m += 1;
            }
            _ => {}
        }
    }

    format!("{:.3}%", m as f32 / w as f32 * 100.0)
}