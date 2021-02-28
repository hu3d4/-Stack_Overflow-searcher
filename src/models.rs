use crate::schema::*;
use actix_web::HttpRequest;
use askama::Template;
use serde::Deserialize;

#[derive(Debug, Queryable)]
pub struct History {
    pub id: i32,
    pub username: String,
    pub input: String,
    pub done: bool,
}

#[derive(Debug, Queryable)]
pub struct User {
    pub userid: i32,
    pub username: String,
    pub login_status: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name = "histories"]
pub struct GetHistory {
    pub input: String,
    pub username: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct GetUser {
    pub username: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "histories"]
pub struct DeleteHistory {
    pub id: i32,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct HistoryTemplate {
    pub entries: Vec<History>,
}

#[derive(Template)]
#[template(path = "authenticated.html")]
pub struct UserHistoryTemplate {
    pub history: HistoryTemplate,
    pub user: String,
}

pub struct UserValue<'a> {
    pub req: HttpRequest,
    pub username: &'a str,
}

impl<'a> UserValue<'a> {
    pub fn get_username(&self) -> String {
        let uservalue = self
            .req
            .match_info()
            .get(&self.username)
            .expect("Failed to load user information.");
        let user = uservalue.to_string();
        user
    }
}
