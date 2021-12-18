use actix_web::{get, App, HttpResponse, HttpServer};
use cyan_nlg;
use std::io::Result;

pub(crate) async fn run_actix() -> Result<()> {
    let host = "127.0.0.1";
    let port = "51440";
    println!("Server running at http://{}:{}", host, port);
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(summarize)
            .service(tokenize)
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Root")
}

#[get("/summarize")]
async fn summarize() -> HttpResponse {
    let text = cyan_nlg::samples::SHORT;
    let body = cyan_nlg::summarize(text).await;
    HttpResponse::Ok().body(body)
}

#[get("/tokenize")]
async fn tokenize() -> HttpResponse {
    let text = cyan_nlg::samples::SHORT;
    let body = cyan_nlg::tokenize(text).await;
    HttpResponse::Ok().body(body)
}
