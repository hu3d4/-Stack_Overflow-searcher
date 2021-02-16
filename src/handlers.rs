use crate::db;
use crate::errors::AppError;
use crate::models::*;
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

pub async fn authenticated(req: HttpRequest) -> Result<impl Responder, AppError> {
    let entries = db::show_history()?;
    let history = HistoryTemplate { entries };
    let uservalue = req
        .match_info()
        .get("username")
        .expect("Failed to load user information.");
    let user = uservalue.to_string();

    let html = UserHistoryTemplate { history, user };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

// pub async fn index(req: HttpRequest) -> Result<impl Responder, AppError> {
//     let uservalue = req
//         .match_info()
//         .get("username")
//         .expect("Failed to load user information.");
//     let user_name = uservalue.to_string();
//     let entries = db::show_history()?;
//     let html = HistoryTemplate { entries };
//     hikaku(user_name, &html);
//     let response_body = html.render()?;
//     Ok(HttpResponse::Ok()
//         .content_type("text/html")
//         .body(response_body))
// }

// fn hikaku(name: String, html: &HistoryTemplate) {
//     for i in &html.entries {
//         println!("{:?}", i);
//     }
//     println!("{}", name);
// }

// pub async fn index_user(req: HttpRequest) -> Result<impl Responder, AppError> {
//     let uservalue = req
//         .match_info()
//         .get("username")
//         .expect("Failed to load user information.");
//     let user_name = uservalue.to_string();
//     println!("index_user{}", user_name);
//     let html = UserTemplate { user_name };
//     let response_body = html.render()?;
//     Ok(HttpResponse::Ok()
//         .content_type("text/html")
//         .body(response_body))
// }

pub async fn get_history(form: web::Form<GetHistory>) -> Result<impl Responder, AppError> {
    let input = form.input.clone();
    let username = form.username.clone();
    println!("{}", input);
    println!("{}", username);
    db::get_history(input, username)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

pub async fn get_user(form: web::Form<GetUser>) -> Result<impl Responder, AppError> {
    let user_name = form.username.clone();
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, format!("/get_user/{}", user_name))
        .finish())
}

pub async fn delete_history(req: HttpRequest) -> Result<impl Responder, AppError> {
    let uservalue = req
        .match_info()
        .get("username")
        .expect("Failed to load user information.");
    let user_name = uservalue.to_string();
    db::delete_all_history(&user_name)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, format!("/get_user/{}", user_name))
        .finish())
}

pub async fn delete_single_history(
    req: HttpRequest,
    form: web::Form<DeleteHistory>,
) -> Result<impl Responder, AppError> {
    let uservalue = req
        .match_info()
        .get("username")
        .expect("Failed to load user information.");
    let user_name = uservalue.to_string();
    let id = form.id;
    db::delete_single_history(id)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, format!("/get_user/{}", user_name))
        .finish())
}
