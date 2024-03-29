mod server;
mod utils;

use actix_web;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    server::run().await
}
