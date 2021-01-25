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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/get", web::post().to(add_history))
            .route("/delete", web::post().to(delete_history))
            .route("/one_delete", web::get().to(delete_one_history))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
