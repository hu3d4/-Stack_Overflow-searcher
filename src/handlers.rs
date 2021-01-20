use crate::db::*;
use crate::errors::AppError;
use crate::models::{AddHistory, IndexTemplate};
use actix_web::{http::header, web, HttpResponse, Responder};
use askama::Template;

pub async fn get_history(form: web::Form<AddHistory>) -> Result<impl Responder, AppError> {
    let input = form.input.clone();
    add_history(input);
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
