use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder};
use crate::utils::{parse, respond, join_abstract, plot_ngram, plot_token, Req, join_reference};
use cyan_nlg::rouge::Rouge;
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
    let (src, _, n) = parse(&data);
    let (abs, ref2, ref3) = join_abstract(src).await;
    let bleu = plot_ngram(src, &abs, n).await;
    let rouge = join_reference(&abs, src, &ref2, &ref3).await;
    println!("{:?}", rouge);
    plot_token(src, &abs).await;
    respond(abs.to_string(), bleu, rouge)
}

async fn calculate(data: web::Json<Req>) -> impl Responder {
    let (src, abs, n) = parse(&data);
    let bleu = plot_ngram(src, abs, n).await;
    respond(abs.to_string(), bleu, Rouge::new())
}
