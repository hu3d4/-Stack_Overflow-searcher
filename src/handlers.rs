use crate::db::*;
use crate::errors::MyError;
use crate::models::IndexTemplate;
use actix_web::{HttpResponse, Responder};
use askama::Template;

// pub async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("hello world")
// }

pub async fn index() -> Result<impl Responder, MyError> {
    let entries = show_history();

    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}
