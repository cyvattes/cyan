mod ngram;

use futures::join;
use tokio::task::spawn_blocking;

pub(crate) const NGRAM_FIELD_LENGTH: u32 = 10;

pub async fn plot(src: &Vec<String>, abs: &Vec<String>) {
    join!(
        plot_n_grams("ng_src", src),
        plot_n_grams("ng_abs", abs),
        plot_freq(src, abs),
    );
}

async fn plot_n_grams(name: &'static str, text: &Vec<String>) {
    let t = text.to_owned();
    spawn_blocking(move || {
        let top = ngram::get_top_n(t, NGRAM_FIELD_LENGTH);
        ngram::plot(name, &top);
    })
        .await
        .unwrap();
}

async fn plot_freq(_src: &Vec<String>, _abs: &Vec<String>) {
    spawn_blocking(move || {
        // 1: calculate POS for src, abs
        // 2: graph
    })
        .await
        .unwrap();
}
