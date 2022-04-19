#[macro_use] extern crate rocket;

pub mod logger;
pub mod configs;
pub mod connection;
pub mod utils;
pub mod models;
pub mod handlers;

use rocket::figment::util::map;
use connection::PostgresPool;
use handlers::{
    healthcheck,
    template_other,
};

#[launch]
fn rocket() -> _ {
    logger::setup_logger().expect("Logger failed to initialize");

    dotenv::dotenv().ok();

    let figment = rocket::Config::figment()
        .merge(("port", *configs::PORT))
        .merge(("databases", map!["postgres" => map![
            "url" => configs::DATABASE_URL.to_owned()
        ]]));

    rocket::custom(figment)
        .attach(PostgresPool::fairing())
        .mount("/api", routes![
            healthcheck::get,
            template_other::get,
        ])
}
