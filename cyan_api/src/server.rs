use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use cyan_nlg;
use std::io::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct Request {
    src: String,
    abs: String,
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

async fn summarize(data: web::Json<Request>) -> impl Responder {
    let src = std::str::from_utf8(&*data.src.as_ref()).expect("Invalid request");
    let resp = cyan_nlg::summarize(src).await;
    HttpResponse::Ok().body(resp)
}

async fn calculate(data: web::Json<Request>) -> impl Responder {
    let src = std::str::from_utf8(&*data.src.as_ref()).expect("Invalid request");
    // let abs = std::str::from_utf8(&*data.abs.as_ref()).expect("Invalid request");
    let resp = cyan_nlg::tokenize(src).await;
    HttpResponse::Ok().body(resp)
}
