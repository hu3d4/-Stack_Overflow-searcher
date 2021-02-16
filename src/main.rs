mod db;
mod errors;
mod handlers;
mod models;
mod schema;

use crate::handlers::*;
use actix_files as fs;
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
            .route("/get_history", web::post().to(get_history))
            .route("/get_user", web::post().to(get_user))
            .route("/get_user/{username}", web::get().to(index_user))
            .route("/get_user/{username}", web::get().to(index))
            .route("/delete/{username}", web::post().to(delete_history))
            .route("/delete_one", web::post().to(delete_single_history))
            .service(fs::Files::new("/", "/static"))
    })
    .listen(listener)?
    .run()
    .await
}
