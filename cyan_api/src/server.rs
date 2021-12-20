use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use cyan_nlg;
use std::io::Result;

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
                Files::new("/", "cyan_api/web/")
                .index_file("templates/index.html")
            )
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}

async fn summarize(body: web::Bytes) -> impl Responder {
    let data = std::str::from_utf8(&*body).expect("Invalid request");
    let resp = cyan_nlg::summarize(data).await;
    HttpResponse::Ok().body(resp)
}

// #[get("/summarize")]
// async fn summarize_get() -> impl Responder {
//     let text = cyan_nlg::samples::SHORT;
//     let body = cyan_nlg::summarize(text).await;
//     HttpResponse::Ok().body(body)
// }

// #[get("/tokenize")]
// async fn tokenize() -> impl Responder {
//     let text = cyan_nlg::samples::SHORT;
//     let body = cyan_nlg::tokenize(text).await;
//     HttpResponse::Ok().body(body)
// }
