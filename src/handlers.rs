use crate::db::*;
use crate::errors::AppError;
use crate::models::{AddHistory, IndexTemplate, Register};
extern crate params;
use crate::diesel::RunQueryDsl;
use crate::schema::*;

use actix_web::{get, http::header, web, HttpResponse, Responder};
use askama::Template;

pub async fn index() -> Result<impl Responder, AppError> {
    let entries = show_history();
    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

// pub async fn add_history(params: web::Form<AddHistory<'_>>) -> Result<impl Responder, AppError> {
//     let connection = establish_connection();
//     let content = &params.input;
//     create_post(&connection, content);
//     Ok(HttpResponse::SeeOther()
//         .header(header::LOCATION, "/")
//         .finish())
// }

// pub async fn add_history(form: web::Form<AddHistory>) -> impl Responder {
//     format!("Hello {}", form.input)
// }

// pub async fn register(form: web::Form<Register>) -> impl Responder {
//     format!("Hello {} from {}!", form.username, form.country)
// }

pub async fn add_history(form: web::Form<AddHistory>) -> Result<impl Responder, AppError> {
    let connection = establish_connection();
    let input = form.input.clone();

    // create_post(&connection, input);
    // create_post(&connection, form.input);

    // pub fn create_post(conn: &PgConnection, input: String) -> History {
    let history_entory = AddHistory { input };
    diesel::insert_into(history::table)
        .values(&history_entory)
        .execute(&connection)
        .expect("Error saving new post");

    // format!("Hello {}", input);
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

// pub async fn register(form: web::Form<Register>) -> impl Responder {
//     format!("Hello {} from {}!", form.username, form.country)
// }

// use crate::models::History;
// use diesel::pg::PgConnection;

// fn create_post(conn: &PgConnection, input: String) -> History {
//     let history_entory = AddHistory { input };
//     diesel::insert_into(history::table)
//         .values(&history_entory)
//         .get_result(conn)
//         .expect("Error saving new post")
// }

// pub struct Person {
//     pub name: String,
// }

// impl Person {
//     pub fn new(name: impl Into<String>) -> Person {
//         Person { name: name.into() }
//     }
// }
