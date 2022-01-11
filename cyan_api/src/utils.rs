use actix_web::{web, HttpResponse, Responder};
use cyan_nlg::utils::Config;
use cyan_vis::{self, utils::TextSource};
use futures::{join, try_join};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct Req {
    src: String,
    abs: String,
    n: String,
}

#[derive(Debug, Serialize)]
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

pub(crate) fn respond(abs: String, bleu: String, rouge: String) -> impl Responder {
    let resp = Resp {
        abs,
        bleu,
        rouge,
    };
    format!("{:?}", resp);
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

    let bleu = cyan_nlg::bleu(&src_ngrams, &abs_ngrams);
    format!("{:.3}%", bleu * 100.0)
}

pub(crate) async fn plot_rouge(abs: &str, ref1: &str, ref2: &str, ref3: &str) -> String {
    let abs = cyan_nlg::strip(abs);
    let ref1 = cyan_nlg::strip(ref1);
    let ref2 = cyan_nlg::strip(ref2);
    let ref3 = cyan_nlg::strip(ref3);

    let mut f1_total: f32 = 0.0;
    let mut matrix = [[[0.0 as f32; 3]; 4]; 3];
    for n in 0..=3 {
        let (
            abs_ngrams,
            ref1_ngrams,
            ref2_ngrams,
            ref3_ngrams,
        ) = match try_join!(
            cyan_nlg::ngramize(&abs, n+1),
            cyan_nlg::ngramize(&ref1, n+1),
            cyan_nlg::ngramize(&ref2, n+1),
            cyan_nlg::ngramize(&ref3, n+1),
        ) {
            Ok(v) => v,
            Err(_) => (
                vec![String::new()],
                vec![String::new()],
                vec![String::new()],
                vec![String::new()],
            ),
        };

        // recall[ref1..=ref3][n1..=n4]
        match try_join!(
            cyan_nlg::recall(&abs_ngrams, &ref1_ngrams),
            cyan_nlg::recall(&abs_ngrams, &ref2_ngrams),
            cyan_nlg::recall(&abs_ngrams, &ref3_ngrams),
        ) {
            Ok((a, b, c)) => {
                matrix[0][n][0]= a;
                matrix[0][n][1]= b;
                matrix[0][n][2]= c;
            },
            Err(_) => {
                matrix[0][n][0] = 0.0;
                matrix[0][n][1] = 0.0;
                matrix[0][n][2] = 0.0;
            },
        };

        // precision[ref1..=ref3][n1..=n4]
        match try_join!(
            cyan_nlg::precision(&abs_ngrams, &ref1_ngrams),
            cyan_nlg::precision(&abs_ngrams, &ref2_ngrams),
            cyan_nlg::precision(&abs_ngrams, &ref3_ngrams),
        ) {
            Ok((a, b, c)) => {
                matrix[1][n][0] = a;
                matrix[1][n][1] = b;
                matrix[1][n][2] = c;
            },
            Err(_) => {
                matrix[1][n][0] = 0.0;
                matrix[1][n][1] = 0.0;
                matrix[1][n][2] = 0.0;
            },
        };

        // f1_means[ref1..=ref3][n1..=n4]
        for r in 0..=2 {
            let f1 = cyan_nlg::rouge(
                matrix[0][n][r],
                matrix[1][n][r]
            );
            matrix[2][n][r] = f1;
            f1_total += f1;
        }

    };

    let rouge = f1_total / 12.0; // sum of ROUGE on 3 REF and N(1..=4)
    cyan_vis::plot_rouge(matrix).await;
    format!("{:.3}%", rouge * 100.0)
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
