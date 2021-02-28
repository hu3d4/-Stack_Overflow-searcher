use crate::db;
use crate::errors::AppError;
use crate::models::*;
use actix_web::{http::header, web, HttpRequest, HttpResponse, Responder};
use askama::Template;

pub async fn index() -> Result<impl Responder, AppError> {
    let text = "text".to_string();
    let entries = db::show_history(&text)?;
    let html = HistoryTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

pub async fn authenticated(req: HttpRequest) -> Result<impl Responder, AppError> {
    let uservalue = UserValue {
        req: req,
        username: &"username",
    };
    let user = uservalue.get_username();
    let entries = db::show_history(&user)?;
    let history = HistoryTemplate { entries };

    let html = UserHistoryTemplate { history, user };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

pub async fn get_history(form: web::Form<GetHistory>) -> Result<impl Responder, AppError> {
    let input = form.0.input;
    let username = form.0.username;
    db::get_history(input, username)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

pub async fn get_user(form: web::Form<GetUser>) -> Result<impl Responder, AppError> {
    let user_name = form.0.username;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, format!("/get_user/{}", user_name))
        .finish())
}

pub async fn delete_history(req: HttpRequest) -> Result<impl Responder, AppError> {
    let uservalue = UserValue {
        req: req,
        username: &"username",
    };
    let user_name = uservalue.get_username();
    db::delete_all_history(&user_name)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, format!("/get_user/{}", user_name))
        .finish())
}

pub async fn delete_single_history(
    req: HttpRequest,
    form: web::Form<DeleteHistory>,
) -> Result<impl Responder, AppError> {
    let uservalue = UserValue {
        req: req,
        username: &"username",
    };
    let user_name = uservalue.get_username();
    let id = form.id;
    db::delete_single_history(id)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, format!("/get_user/{}", user_name))
        .finish())
}
