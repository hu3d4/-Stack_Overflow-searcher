use crate::schema::*;

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
pub struct IndexTemplate {
    pub entries: Vec<History>,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct UserTemplate {
    pub user_name: String,
}
// userの let html = IndexTemplate { entries };
// let response_body = html.render()?;
// を書かないといけない
