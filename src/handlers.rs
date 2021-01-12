use crate::db::*;
use crate::errors::MyError;
use crate::models::{AddHistory, IndexTemplate};
use actix_web::{http::header, web, HttpResponse, Responder};
use askama::Template;

pub async fn add_history() -> Result<impl Responder, MyError> {
    let connection = establish_connection();
    let input = "Hello World!";
    // let test = AddHistory { input };
    // let test = AddHistory { input: "hello" };
    create_post(&connection, input);
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

// pub async fn add_history(params: web::Form<AddHistory<'a>>) -> Result<impl Responder, MyError> {
//     let connection = establish_connection();
//     let content = &params.input;
//     // connection.get_result("INSERT INTO todo (text) VALUES (?)", &[&params.text]);
//     create_post(&connection, content);
//     // let responce_history = create_post(&connection, content);
//     Ok(HttpResponse::SeeOther()
//         .header(header::LOCATION, "/")
//         .finish())
//     // Ok(HttpResponse::Ok()
//     //     .content_type("application")
//     //     .body(responce_history))
// }
