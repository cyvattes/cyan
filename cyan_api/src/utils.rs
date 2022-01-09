use actix_web::{web, HttpResponse, Responder};
use cyan_vis::{self, utils::TextSource};
use futures::{join, try_join};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct Req {
    src: String,
    abs: String,
    n: String,
}


#[derive(Serialize)]
struct Resp {
    abs: String,
    bleu: String,
    rouge: String,
}

pub(crate) fn parse(data: &web::Json<Req>) -> (&str, &str, usize) {
    (
        data.src.as_str(),
        data.abs.as_str(),
        data.n.parse().unwrap(),
    )
}

pub(crate) fn respond(abs: String, bleu: String) -> impl Responder {
    let resp = Resp {
        abs,
        bleu,
        rouge: String::new(),
    };
    let json = serde_json::to_string(&resp).unwrap();
    HttpResponse::Ok().body(json)
}

pub(crate) async fn join_abstract(src: &str) -> String {
    // TODO: maybe add join! macro here?
    let (
        abs,
    ) = match try_join!(
        cyan_nlg::summarize(src)
    ) {
        Ok(v) => v,
        Err(_) => (
            String::new(),
        ),
    };

    abs
}

pub(crate) async fn plot_ngram(src: &str, abs: &str, n: usize) -> String {
    let src = cyan_nlg::strip(src);
    let abs = cyan_nlg::strip(abs);

    let (
        src_ngrams,
        abs_ngrams,
    ) = match try_join!(
        cyan_nlg::ngramize(&src, n),
        cyan_nlg::ngramize(&abs, n),
    ) {
        Ok(v) => v,
        Err(_) => (
            vec![String::new()],
            vec![String::new()],
        ),
    };

    join!(
        cyan_vis::plot_ngram(TextSource::SRC, &src_ngrams),
        cyan_vis::plot_ngram(TextSource::ABS, &abs_ngrams),
    );

    cyan_nlg::bleu(&src_ngrams, &abs_ngrams)
}

pub(crate) async fn plot_token(src: &str, abs: &str) {
    let src = cyan_nlg::strip(src);
    let abs = cyan_nlg::strip(abs);

    let (
        src_pos,
        abs_pos,
    ) = match try_join!(
        cyan_nlg::tokenize(&src),
        cyan_nlg::tokenize(&abs),
    ) {
        Ok(v) => v,
        Err(_) => (
            vec![(String::new(), 0)],
            vec![(String::new(), 0)],
        ),
    };

    cyan_vis::plot_token(&src_pos, &abs_pos).await;
}
