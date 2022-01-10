use actix_web::{web, HttpResponse, Responder};
use cyan_nlg::{bleu::Bleu, rouge::Rouge, utils::Config};
use cyan_vis::{self, utils::TextSource};
use futures::{join, try_join};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Deserialize)]
pub(crate) struct Req {
    src: String,
    abs: String,
    n: String,
}

#[derive(Serialize)]
struct Resp {
    abs: String,
    bleu: Bleu,
    rouge: Rouge,
}

pub(crate) fn parse(data: &web::Json<Req>) -> (&str, &str, usize) {
    (
        data.src.as_str(),
        data.abs.as_str(),
        data.n.parse().unwrap(),
    )
}

pub(crate) fn respond(abs: String, bleu: Bleu, rouge: Rouge) -> impl Responder {
    let resp = Resp {
        abs,
        bleu,
        rouge,
    };
    let json = serde_json::to_string(&resp).unwrap();
    HttpResponse::Ok().body(json)
}

pub(crate) async fn join_abstract(src: &str) -> (String, String, String) {
    let (
        abs1,
        abs2,
        abs3,
    ) = match try_join!(
        cyan_nlg::summarize(src, Config::BART),
        cyan_nlg::summarize(src, Config::T5GN),
        cyan_nlg::summarize(src, Config::PNET),
    ) {
        Ok(v) => v,
        Err(_) => (
            String::new(),
            String::new(),
            String::new(),
        ),
    };

    (abs1, abs2, abs3)
}

pub(crate) async fn join_reference(abs: &str, ref1: &str, ref2: &str, ref3: &str) -> Rouge {
    let t = Instant::now();
    println!("ROUGE started");
    let abs = cyan_nlg::strip(abs);
    let ref1 = cyan_nlg::strip(ref1);
    let ref2 = cyan_nlg::strip(ref2);
    let ref3 = cyan_nlg::strip(ref3);

    let mut recall: f32 = 0.0;
    let mut precision: f32 = 0.0;
    for n in 1..=4 {
        let (
            abs_ngrams,
            ref1_ngrams,
            ref2_ngrams,
            ref3_ngrams,
        ) = match try_join!(
        cyan_nlg::ngramize(&abs, n),
        cyan_nlg::ngramize(&ref1, n),
        cyan_nlg::ngramize(&ref2, n),
        cyan_nlg::ngramize(&ref3, n),
    ) {
            Ok(v) => v,
            Err(_) => (
                vec![String::new()],
                vec![String::new()],
                vec![String::new()],
                vec![String::new()],
            ),
        };

        recall += match try_join!(
            cyan_nlg::recall(&abs_ngrams, &ref1_ngrams),
            cyan_nlg::recall(&abs_ngrams, &ref2_ngrams),
            cyan_nlg::recall(&abs_ngrams, &ref3_ngrams),
        ) {
            Ok((a, b, c)) => (a + b + c) / 3.0,
            Err(_) => 0.0,
        };

        precision += match try_join!(
            cyan_nlg::precision(&abs_ngrams, &ref1_ngrams),
            cyan_nlg::precision(&abs_ngrams, &ref2_ngrams),
            cyan_nlg::precision(&abs_ngrams, &ref3_ngrams),
        ) {
            Ok((a, b, c)) => (a + b + c) / 3.0,
            Err(_) => 0.0,
        };
    };
    recall /= 4.0;
    precision /= 4.0;
    println!("ROUGE finished in {:?}", t.elapsed());
    cyan_nlg::rouge(recall, precision)
}

pub(crate) async fn plot_ngram(src: &str, abs: &str, n: usize) -> Bleu {
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
