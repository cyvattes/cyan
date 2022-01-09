mod ngram;
use futures::join;
use tokio::task::spawn_blocking;

pub(crate) const NGRAM_FIELD_LENGTH: u32 = 10;

pub async fn plot_freq(src: &Vec<String>, abs: &Vec<String>) {
    spawn_blocking(move || {
        // 1: calculate POS for src, abs
        // 2: graph
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
