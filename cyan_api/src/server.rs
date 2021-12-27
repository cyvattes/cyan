use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use cyan_nlg::{self, tokenizer::NGram};
use std::io::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Req {
    src: String,
    abs: String,
}

#[derive(Serialize)]
struct Resp {
    src_ngram: NGram,
    abs_ngram: NGram,
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
    let src = std::str::from_utf8(&*data.src.as_ref()).expect("Invalid request");
    let resp = cyan_nlg::summarize(src).await;
    let json = serde_json::to_string(&resp).unwrap();
    HttpResponse::Ok().body(json)
}

async fn calculate(data: web::Json<Req>) -> impl Responder {
    let src = std::str::from_utf8(&*data.src.as_ref()).expect("Invalid request");
    let abs = std::str::from_utf8(&*data.abs.as_ref()).expect("Invalid request");
    let resp = Resp {
        src_ngram: cyan_nlg::tokenize(src).await,
        abs_ngram: cyan_nlg::tokenize(abs).await,
    };

    let json = serde_json::to_string(&resp).unwrap();
    HttpResponse::Ok().body(json)
}
