mod db;
mod errors;
mod handlers;
mod models;
mod schema;

use crate::handlers::*;
use actix_web::{web, App, HttpServer};

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            // .route("http://google.com/cse", web::get().to(history))
            .route("/add", web::get().to(add_history))
        // .route("/test", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
