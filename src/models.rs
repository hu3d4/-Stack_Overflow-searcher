use crate::schema::*;

use askama::Template;
use serde::Deserialize;

#[derive(Debug, Queryable)]
pub struct History {
    pub id: i32,
    pub userid: i32,
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
