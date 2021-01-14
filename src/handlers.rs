use crate::db::*;
use crate::errors::AppError;
use crate::models::{AddHistory, IndexTemplate};

use actix_web::{get, http::header, web, HttpResponse, Responder};
use askama::Template;

// pub async fn add_history() -> Result<impl Responder, AppError> {
//     let connection = establish_connection();
//     let input = "Hello World!";
//     create_post(&connection, input);
//     Ok(HttpResponse::SeeOther()
//         .header(header::LOCATION, "/")
//         .finish())
// }

pub async fn index() -> Result<impl Responder, AppError> {
    let entries = show_history();
    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

pub async fn add_history(params: web::Form<AddHistory<'_>>) -> Result<impl Responder, AppError> {
    let connection = establish_connection();
    let content = &params.input;
    create_post(&connection, content);
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}
