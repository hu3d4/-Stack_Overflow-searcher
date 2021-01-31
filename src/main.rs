mod db;
mod errors;
mod handlers;
mod models;
mod schema;

use crate::handlers::*;
use actix_web::{web, App, HttpServer};

#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
extern crate dotenv;

use diesel_migrations::embed_migrations;

embed_migrations!("../migrations");

fn maidn() {
    use crate::db::establish_connection;
    let connection = establish_connection();

    // This will run the necessary migrations.
    embedded_migrations::run(&connection);
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/get", web::post().to(add_history))
            .route("/delete", web::post().to(delete_history))
            .route("/delete_one", web::post().to(delete_one_history))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
