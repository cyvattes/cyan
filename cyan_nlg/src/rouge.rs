use crate::bleu::naive_bleu;
pub(crate) fn from(recall: f32, precision: f32) -> f32 {
    f1(recall, precision)
}

pub(crate) fn recall(abs: &Vec<String>, rf: &Vec<String>) -> f32 {
    let w = rf.len() as f32;  // Total values found in REF
    naive_bleu(abs, rf, w)
}

pub(crate) fn precision(abs: &Vec<String>, rf: &Vec<String>) -> f32 {
    let w = abs.len() as f32;  // Total values found in ABS
    naive_bleu(abs, rf, w)
}

fn f1(recall: f32, precision: f32) -> f32 {
    if (precision + recall) == 0.0 {
        return 0.0;
    }
    2.0 * ((precision * recall) / (precision + recall))
}

