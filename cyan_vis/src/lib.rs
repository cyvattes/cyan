mod freq;
mod ngram;

use futures::join;
use tokio::task::spawn_blocking;

pub(crate) const NGRAM_FIELD_LENGTH: u32 = 10;

pub async fn plot_freq(src: &Vec<(String, u32)>, abs: &Vec<(String, u32)>) {
    let s = src.to_owned();
    let a = abs.to_owned();
    spawn_blocking(move || {
        freq::plot(&s, &a);
    })
        .await
        .unwrap();
}

pub async fn plot_ngrams(name: &'static str, text: &Vec<String>) {
    let t = text.to_owned();
    spawn_blocking(move || {
        let top = ngram::get_top_n(t, NGRAM_FIELD_LENGTH);
        ngram::plot(name, &top);
    })
        .await
        .unwrap();
}
