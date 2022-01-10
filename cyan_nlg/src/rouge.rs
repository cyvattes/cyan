use crate::bleu::naive_bleu;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Rouge {
    recall: String,
    precision: String,
    f1: String,
}

impl Rouge {
    pub fn new() -> Rouge {
        Rouge {
            recall: String::new(),
            precision: String::new(),
            f1: String::new(),
        }
    }

    pub(crate) fn from(recall: f32, precision: f32) -> Rouge {
        Rouge {
            recall: recall.to_string(),
            precision: precision.to_string(),
            f1: format!("{:.3}%", f1(recall, precision)),
        }
    }
}

pub(crate) fn recall(abs: &Vec<String>, rf: &Vec<String>) -> f32 {
    let w = rf.len() as f32;  // Total values found in REF
    naive_bleu(abs, rf, w)
}

pub(crate) fn precision(abs: &Vec<String>, rf: &Vec<String>) -> f32 {
    let w = abs.len() as f32;  // Total values found in ABS
    naive_bleu(abs, rf, w)
}

pub fn f1(recall: f32, precision: f32) -> f32 {
    if (precision + recall) == 0.0 {
        return 0.0;
    }
    2.0 * ((precision * recall) / (precision + recall))
}

