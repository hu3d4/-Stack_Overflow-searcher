use crate::db;
use crate::errors::AppError;
use crate::models::DeleteHistory;
use crate::models::{AddHistory, IndexTemplate};
use actix_web::{http::header, web, HttpResponse, Responder};
use askama::Template;

pub async fn index() -> Result<impl Responder, AppError> {
    let entries = db::show_history()?;
    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

pub async fn add_history(form: web::Form<AddHistory>) -> Result<impl Responder, AppError> {
    let input = form.input.clone();
    db::add_history(input)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

pub async fn delete_history() -> Result<impl Responder, AppError> {
    db::delete_history()?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

pub async fn delete_one_history(
    form: web::Form<DeleteHistory>,
) -> Result<impl Responder, AppError> {
    let id = form.id;
    db::delete_one_history(id);
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}
