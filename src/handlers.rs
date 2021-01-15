use crate::db::*;
use crate::errors::AppError;
use crate::models::{AddHistory, IndexTemplate};
use actix_web::{http::header, post, web, HttpResponse, Responder};
use askama::Template;

#[post("/add")]
pub async fn add_history(form: web::Form<AddHistory>) -> Result<impl Responder, AppError> {
    let connection = establish_connection();
    let input = form.input.clone();
    create_post(&connection, input);
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

pub async fn index() -> Result<impl Responder, AppError> {
    let entries = show_history();
    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}
