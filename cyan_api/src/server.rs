use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::str;
use cyan_nlg;
use std::io::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Req {
    src: String,
    abs: String,
    n: String,
}

#[derive(Serialize)]
struct Resp {
    abs: String,
    bleu: String,
    src_ngram: Vec<String>,
    abs_ngram: Vec<String>,
}

pub(crate) async fn run() -> Result<()> {
    let host = "127.0.0.1";
    let port = "51440";
    println!("Service running at http://{}:{}", host, port);
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/summarize")
                    .route(web::post().to(summarize))
            )
            .service(
                web::resource("/calculate")
                    .route(web::post().to(calculate))
            )
            .service(
                Files::new("/", "cyan_api/web/")
                .index_file("templates/index.html")
            )
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}

async fn summarize(data: web::Json<Req>) -> impl Responder {
    let src = str::from_utf8(&*data.src.as_ref());
    let s = cyan_nlg::summarize(src).await;

    let abs = str::from_utf8(s.as_ref());
    let n: usize = str::from_utf8(&*data.n.as_ref())
        .parse()
        .unwrap();

    let src_ngram = cyan_nlg::tokenize(src, n).await;
    let abs_ngram = cyan_nlg::tokenize(abs, n).await;

    let resp = Resp {
        abs: abs.to_string(),
        bleu: cyan_nlg::bleu(&src_ngram, &abs_ngram),
        src_ngram,
        abs_ngram,
    };

    let json = serde_json::to_string(&resp).unwrap();
    HttpResponse::Ok().body(json)
}

async fn calculate(data: web::Json<Req>) -> impl Responder {
    let src = str::from_utf8(&*data.src.as_ref());
    let abs = str::from_utf8(&*data.abs.as_ref());
    let n: usize = str::from_utf8(&*data.n.as_ref())
        .parse()
        .unwrap();

    let src_ngram = cyan_nlg::tokenize(src, n).await;
    let abs_ngram = cyan_nlg::tokenize(abs, n).await;

    let resp = Resp {
        abs: String::new(),
        bleu: cyan_nlg::bleu(&src_ngram, &abs_ngram),
        src_ngram,
        abs_ngram,
    };

    let json = serde_json::to_string(&resp).unwrap();
    HttpResponse::Ok().body(json)
}
