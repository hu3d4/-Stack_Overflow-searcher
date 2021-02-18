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

pub async fn authenticated(req: HttpRequest) -> Result<HttpResponse, AppError> {
    let uservalue = req
        .match_info()
        .get("username")
        .expect("Failed to load user information.");
    let user = uservalue.to_string();

    let entries = db::show_history(&user)?;
    let history = HistoryTemplate { entries };

    let html = UserHistoryTemplate { history, user };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

pub async fn get_history(form: web::Form<GetHistory>) -> Result<impl Responder, AppError> {
    let input = form.input.clone();
    let username = form.username.clone();
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

pub async fn delete_history(req: HttpRequest) -> Result<HttpResponse, AppError> {
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

// async fn index(req: HttpRequest) -> HttpResponse {
//     if let Some(hdr) = req.headers().get(header::CONTENT_TYPE) {
//         HttpResponse::Ok().into()
//     } else {
//         HttpResponse::BadRequest().into()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, web, App};
    use pretty_assertions::assert_eq;

    use actix_web::{http::StatusCode, HttpResponse};

    #[actix_rt::test]
    async fn test_response() {
        let mut app = test::init_service(
            App::new().route("/delete/{username}", web::post().to(delete_history)),
        )
        .await;

        let req = test::TestRequest::with_uri("/delete/username").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    //     // #[actix_rt::test]
    //     // async fn test_index_ok() {
    //     //     let req = test::TestRequest::with_uri("/get_user/{username}").to_http_request();
    //     //     let resp = authenticated(req).await.unwrap();
    //     //     assert_eq!(resp.status(), http::StatusCode::OK);
    //     // }

    //     use actix_web::{http::StatusCode, HttpResponse};

    //     #[actix_rt::test]
    //     async fn test_response() {
    //         let mut app = test::init_service(
    //             App::new().route("/delete/{username}", web::post().to(delete_history)),
    //         )
    //         .await;

    //         let req = test::TestRequest::with_uri("/delete/username").to_request();
    //         let resp = test::call_service(&mut app, req).await;
    //         assert_ne!(resp.status(), StatusCode::OK);
    //     }

    //     // #[actix_rt::test]
    //     // async fn test_delete_index_ok() {
    //     //     let req = test::TestRequest::with_uri("/delete/{username}").to_http_request();
    //     //     let resp = delete_history(req).await.unwrap();
    //     //     assert_eq!(resp.status(), http::StatusCode::OK);
    //     // }

    //     #[actix_rt::test]
    //     async fn test_index_not_ok() {
    //         use actix_web::HttpMessage;
    //         let payload = GetUser {
    //             username: "jeiow".to_string(),
    //         };
    //         let req = test::TestRequest::post()
    //             .uri("/get_history")
    //             .set_form(&payload)
    //             .to_request();
    //         // let resp = get_user(payload).await;
    //         // let resp = get_user(req).await;
    //         assert_eq!(req.content_type(), "application/x-www-form-urlencoded");
    //         // let resp = get_user(req).await;
    //         // assert_eq!(resp, http::StatusCode::BAD_REQUEST);
    //     }
}
