use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use cyan_nlg;
use cyan_vis;
use futures::try_join;
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
    rouge: String,
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
    let (src, _abs, n) = (
        data.src.as_str(),
        data.abs.as_str(),
        data.n.parse().unwrap(),
    );
    let summary = cyan_nlg::summarize(src).await;
    let abs = summary.as_str();
    set_and_respond(src, abs, n).await
}

async fn calculate(data: web::Json<Req>) -> impl Responder {
    let (src, abs, n) = (
        data.src.as_str(),
        data.abs.as_str(),
        data.n.parse().unwrap(),
    );
    set_and_respond(src, abs, n).await
}

async fn set_and_respond(src: &str, abs: &str, n: usize) -> impl Responder {
    let resp = Resp {
        abs: abs.to_string(),
        bleu: set(src, abs, n).await,
        rouge: String::new(),
    };

    respond(&resp)
}

async fn set(src: &str, abs: &str, n: usize) -> String {
    let (src_ngram, abs_ngram) = match try_join!(
        cyan_nlg::tokenize(src, n),
        cyan_nlg::tokenize(abs, n),
    ) {
        Ok(v) => v,
        Err(_) => (
            vec![String::new()],
            vec![String::new()],
        ),
    };

    cyan_vis::plot(&src_ngram, &abs_ngram).await;
    cyan_nlg::bleu(&src_ngram, &abs_ngram)
}

fn respond(resp: &Resp) -> impl Responder {
    let json = serde_json::to_string(resp).unwrap();
    HttpResponse::Ok().body(json)
}
