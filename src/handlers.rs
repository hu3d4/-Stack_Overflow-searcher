use crate::db::*;
use crate::errors::MyError;
use crate::models::{AddHistory, IndexTemplate};
use actix_web::{http::header, web, HttpResponse, Responder};
use askama::Template;

// pub async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("hello world")
// }

// pub fn history() {
//     let connection = establish_connection();
//     // let test = "hello";
//     fn output() -> &str {
//         let hello = "hello";
//         return &hello;
//     }

//     let input = output();
//     // let test = AddHistory { input };

//     return create_post(&connection, input);
//     // Ok(HttpResponse::SeeOther()
//     //     .header(header::LOCATION, "/")
//     //     .finish())
// }

pub async fn add_history() -> Result<impl Responder, MyError> {
    let connection = establish_connection();
    // let test = AddHistory { input: "hello" };
    let test = "hello";
    // let input = AddHistory { input: "hello" };
    // let input = { input: "hello" };
    // let test = AddHistory { input };
    create_post(&connection, test);
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

pub async fn index() -> Result<impl Responder, MyError> {
    let entries = show_history();
    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}
