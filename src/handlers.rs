use crate::errors::MyError;
use crate::models::{History, IndexTemplate};
use actix_web::{HttpResponse, Responder};
use askama::Template;

// pub async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("hello world")
// }

pub async fn index() -> Result<impl Responder, MyError> {
    let mut entries = Vec::new();

    entries.push(History {
        id: 1,
        input: "First entry".to_string(),
        done: false,
    });
    entries.push(History {
        id: 2,
        input: "Second entry".to_string(),
        done: false,
    });

    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}
