use crate::db;
use crate::errors::AppError;
use crate::models::{
    DeleteHistory, GetHistory,  HistoryTemplate, UserValue,
};
use actix_web::{http::header, web, HttpRequest, HttpResponse, Responder};
use askama::Template;

pub async fn index() -> Result<impl Responder, AppError> {
    let entries = db::show_history()?;
    let html = HistoryTemplate { entries };
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

pub async fn delete_history(req: HttpRequest) -> Result<impl Responder, AppError> {
    let user_value = UserValue(req, "user_name");
    let user_name = user_value.get_user_name();
    db::delete_all_history(&user_name)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

pub async fn delete_single_history(
    form: web::Form<DeleteHistory>,
) -> Result<impl Responder, AppError> {

    let id = form.0.id;
    db::delete_single_history(id)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}
