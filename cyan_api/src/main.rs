mod server;

#[macro_use]
extern crate rocket;
use rocket::Build;

#[launch]
fn launch() -> rocket::Rocket<Build> {
    server::run()
}
