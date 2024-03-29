mod ngram;
mod rouge;
mod token;
pub mod utils;

use crate::utils::{NGRAM_FIELD_LENGTH, TextSource};
use tokio::task::spawn_blocking;

pub async fn plot_ngram(ts: TextSource, text: &Vec<String>) {
    let t = text.to_owned();
    spawn_blocking(move || {
        let top = ngram::get_top_n(t, NGRAM_FIELD_LENGTH);
        ngram::plot(ts, &top);
    })
        .await
        .unwrap();
}

pub async fn plot_rouge(matrix: [[[f32; 3]; 4]; 3]) {
    let m = matrix.to_owned();
    spawn_blocking(move || {
        rouge::plot(m)
    })
        .await
        .unwrap();
}

pub async fn plot_token(src: &Vec<(String, u32)>, abs: &Vec<(String, u32)>) {
    let s = src.to_owned();
    let a = abs.to_owned();
    spawn_blocking(move || {
        token::plot(&s, &a);
    })
        .await
        .unwrap();
}
