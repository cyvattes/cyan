use std::collections::HashMap;
pub(crate) fn from(src: &Vec<String>, abs: &Vec<String>) -> f32 {
    strict_bleu(src, abs)
}

fn strict_bleu(src: &Vec<String>, abs: &Vec<String>) -> f32 {
    let mut m: f32 = 0.0;      // values in ABS found in SRC, capped at M(max)
    let w: f32 = abs.len() as f32;  // Total values found in ABS

    // Loop over SRC, capture M(max) per value
    let mut src_map: HashMap<String, usize> = HashMap::new();
    for ngram in src {
        *src_map.entry(ngram.to_string()).or_insert(0) += 1;
    }

    // Loop over ABS, add to M per matched value
    for ngram in abs {
        match src_map.get_mut(ngram) {
            Some(x) if *x > 0 => {
                *x -= 1;
                m += 1.0;
            },
            _ => {},
        }
    }

    m / w
}

pub fn naive_bleu(abs: &Vec<String>, rf: &Vec<String>, w: f32) -> f32 {
    let mut m: f32 = 0.0;      // Values in ABS found in REF
    let mut ref_map: HashMap<String, bool> = HashMap::new();
    for ngram in rf {
        *ref_map.entry(ngram.to_string()).or_default() = true;
    }

    for ngram in abs {
        match ref_map.get(ngram) {
            Some(true) => m += 1.0,
            _ => {},
        }
    }

    m / w
}
