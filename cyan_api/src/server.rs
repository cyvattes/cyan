use cyan_nlg;
use rocket::{Build, routes};

pub(crate) fn run() -> rocket::Rocket<Build> {
    rocket::build()
        .mount("/", routes![
            index,
            summarize,
            tokenize,
        ])
}

#[get("/")]
fn index() -> String {
    String::from("Root")
}

#[get("/summarize")]
async fn summarize() -> String {
    let text = cyan_nlg::samples::SHORT;
    cyan_nlg::summarize(text).await
}

#[get("/tokenize")]
async fn tokenize() -> String {
    let text = cyan_nlg::samples::SHORT;
    cyan_nlg::tokenize(text).await
}
