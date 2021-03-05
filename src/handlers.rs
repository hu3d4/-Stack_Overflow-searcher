use crate::db;
use crate::errors::AppError;
use crate::models::{
    DeleteHistory, GetHistory, GetUser, HistoryTemplate, UserHistoryTemplate, UserValue,
};
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
    let user_value = UserValue(req, &"user_name");
    let user = user_value.get_user_name();
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
    let user_name = form.0.username;
    db::get_history(input, user_name)?;
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
    let user_value = UserValue(req, &"user_name");
    let user_name = user_value.get_user_name();
    db::delete_all_history(&user_name)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, format!("/get_user/{}", user_name))
        .finish())
}

pub async fn delete_single_history(
    req: HttpRequest,
    form: web::Form<DeleteHistory>,
) -> Result<impl Responder, AppError> {
    let user_value = UserValue(req, &"user_name");
    let user_name = user_value.get_user_name();
    let id = form.id;
    db::delete_single_history(id)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, format!("/get_user/{}", user_name))
        .finish())
}
