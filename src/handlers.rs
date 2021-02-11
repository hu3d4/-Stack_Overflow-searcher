use crate::db;
use crate::errors::AppError;
use crate::models::*;
use actix_web::{http::header, web, HttpResponse, Responder};
use askama::Template;

pub async fn index() -> Result<impl Responder, AppError> {
    let entries = db::show_history()?;
    let user = db::show_user();

    let html = for u in user.iter() {
        for i in u.iter() {
            let e = i.id;
            for h in entries.iter() {
                let a = h.id;
                if e == a {
                    IndexTemplate { entries }
                } else {
                    panic!("jfie");
                };
            }
        }
    };
    // let html = for u in user.iter() {
    //     for i in u.iter() {
    //         let e = i.id;
    //         for h in entries.iter() {
    //             let a = h.id;
    //             if e == a {
    //                 IndexTemplate { entries }
    //             } else {
    //                 panic!("jfie");
    //             };
    //         }
    //     }
    // };

    // let history_id = for h in entries.iter() {
    //     let a = h.id;
    // };

    // let html = match user_id {
    //     history_id => IndexTemplate { entries },
    //     _ => panic!("fjie"),
    // };

    // let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

// pub async fn index_users() -> Result<impl Responder, AppError> {
//     use crate::schema::histories::id;
//     let entries = db::show_history()?;
//     // ユーザーごとに表示する値を変更する。
//     // pub id: i32,
//     // pub input: String,
//     // pub done: bool,
//     for i in entries.iter() {
//         let a = i.id;
//     }
//     let a = entries;
//     let html = ResultTemplate { entries };
//     let response_body = html.render()?;
//     Ok(HttpResponse::Ok()
//         .content_type("text/html")
//         .body(response_body))
// }

pub async fn add_history(form: web::Form<AddHistory>) -> Result<impl Responder, AppError> {
    db::add_history(&form)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

pub async fn add_user(form: web::Form<AddUser>) -> Result<impl Responder, AppError> {
    db::add_user(&form)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

pub async fn delete_history() -> Result<impl Responder, AppError> {
    db::delete_all_history()?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

pub async fn delete_single_history(
    form: web::Form<DeleteHistory>,
) -> Result<impl Responder, AppError> {
    let id = form.id;
    db::delete_single_history(id)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

pub async fn delete_single_user(
    form: web::Form<DeleteHistory>,
) -> Result<impl Responder, AppError> {
    let id = form.id;
    db::delete_single_history(id)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}
