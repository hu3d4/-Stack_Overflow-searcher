mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod schema;

use crate::config::Config;
use crate::handlers::*;
use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env().expect("Server configuration");
    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port))
        .expect("unable to bind TCP listener");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/get", web::post().to(add_history))
            .route("/delete", web::post().to(delete_history))
            .route("/delete_one", web::post().to(delete_single_history))
            .route("/add_user", web::post().to(add_user))
            .route("/register/{id}", web::delete().to(delete_single_user))
            .service(fs::Files::new("/", "/static"))
    })
    .listen(listener)?
    .run()
    .await
}
