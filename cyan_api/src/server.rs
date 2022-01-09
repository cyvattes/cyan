use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use cyan_nlg;
use cyan_vis;
use futures::{join, try_join};
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
    let (src, _abs, n) = parse(&data);
    // TODO: maybe add join! macro here?
    let abs = &cyan_nlg::summarize(src).await.unwrap();
    set_and_respond(src, abs, n).await
}

async fn calculate(data: web::Json<Req>) -> impl Responder {
    let (src, abs, n) = parse(&data);
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
    let src = cyan_nlg::strip(src);
    let abs = cyan_nlg::strip(abs);

    let (
        src_pos,
        abs_pos,
        src_ngrams,
        abs_ngrams,
    ) = match try_join!(
        cyan_nlg::tokenize(&src),
        cyan_nlg::tokenize(&abs),
        cyan_nlg::ngramize(&src, n),
        cyan_nlg::ngramize(&abs, n),
    ) {
        Ok(v) => v,
        Err(_) => (
            vec![(String::new(), 0)],
            vec![(String::new(), 0)],
            vec![String::new()],
            vec![String::new()],
        ),
    };

    join!(
        cyan_vis::plot_freq(&src_pos, &abs_pos),
        cyan_vis::plot_ngrams("ng_src", &src_ngrams),
        cyan_vis::plot_ngrams("ng_abs", &abs_ngrams),
    );

    cyan_nlg::bleu(&src_ngrams, &abs_ngrams)
}

fn respond(resp: &Resp) -> impl Responder {
    let json = serde_json::to_string(resp).unwrap();
    HttpResponse::Ok().body(json)
}

fn parse(data: &web::Json<Req>) -> (&str, &str, usize) {
    (
        data.src.as_str(),
        data.abs.as_str(),
        data.n.parse().unwrap(),
    )
}
