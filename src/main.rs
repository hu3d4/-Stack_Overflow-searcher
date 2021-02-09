mod db;
mod errors;
mod handlers;
mod models;
mod schema;

use crate::handlers::*;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("unable to bind TCP listener");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/get", web::post().to(add_history))
            .route("/delete", web::post().to(delete_history))
            .route("/delete_one", web::post().to(delete_one_history))
            .route("/add_user", web::post().to(add_user))
    })
    .listen(listener)?
    .run()
    .await
}
