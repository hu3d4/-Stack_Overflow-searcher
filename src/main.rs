// mod db;
mod errors;
mod handlers;
mod models;
mod schema;

use crate::handlers::*;
use actix_web::{web, App, HttpServer};

#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::models::{History, HistoryEntry};
use crate::schema::*;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub async fn create_post<'a>(
    conn: &PgConnection,
    input: &'a str, /* , done: &'a bool */
) -> History {
    // use schema::*;
    let history_entory = HistoryEntry { input, /* done */ };
    diesel::insert_into(history::table)
        .values(&history_entory)
        .get_result(conn)
        .expect("Error saving new post")
}

pub async fn output() {
    use crate::diesel::prelude::*;
    use crate::schema::history::dsl::*;

    let connection = establish_connection();
    let results = history
        .filter(done.eq(true))
        .limit(5)
        .load::<History>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.input);
        println!("----------\n");
        // println!("{}", post.body);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
        // .route("/test", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
