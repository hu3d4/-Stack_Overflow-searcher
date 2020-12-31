use crate::errors::MyError;
use crate::models::{IndexTemplate, LogEntry};
use actix_web::{HttpResponse, Responder};
use askama::Template;

// pub async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("hello world")
// }

pub async fn index() -> Result<impl Responder, MyError> {
    let mut entries = Vec::new();

    entries.push(LogEntry {
        id: 1,
        text: "First entry".to_string(),
    });
    entries.push(LogEntry {
        id: 2,
        text: "Second entry".to_string(),
    });

    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}
