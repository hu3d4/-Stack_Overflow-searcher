use crate::db::*;
use crate::errors::MyError;
use crate::models::{History, HistoryEntry, IndexTemplate};
use actix_web::{HttpResponse, Responder};
use askama::Template;

// pub async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("hello world")
// }

pub async fn index() -> Result<impl Responder, MyError> {
    // let mut entries = Vec::new();

    use crate::diesel::prelude::*;
    use crate::schema::history::dsl::*;

    let connection = establish_connection();
    let results = history
        .filter(done.eq(true))
        .limit(5)
        .load::<History>(&connection)
        .expect("Error loading posts");

    // entries.push(results);

    // entries.push(History {
    //     id: 1,
    //     input: "First entry".to_string(),
    //     done: false,
    // });
    // entries.push(History {
    //     id: 2,
    //     input: "Second entry".to_string(),
    //     done: false,
    // });

    let html = IndexTemplate { results };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

// use crate::models::HistoryEntry;
// use crate::schema::*;

// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use dotenv::dotenv;
// use std::env;

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
// }

// pub async fn create_post<'a>(
//     conn: &PgConnection,
//     input: &'a str, /* , done: &'a bool */
// ) -> History {
//     // use schema::*;
//     let history_entory = HistoryEntry { input, /* done */ };
//     diesel::insert_into(history::table)
//         .values(&history_entory)
//         .get_result(conn)
//         .expect("Error saving new post")
// }

// pub async fn output() {
//     use crate::diesel::prelude::*;
//     use crate::schema::history::dsl::*;

//     let connection = establish_connection();
//     let results = history
//         .filter(done.eq(true))
//         .limit(5)
//         .load::<History>(&connection)
//         .expect("Error loading posts");

//     println!("Displaying {} posts", results.len());
//     for post in results {
//         println!("{}", post.input);
//         println!("----------\n");
//         // println!("{}", post.body);
//     }
// }
