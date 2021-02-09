use crate::schema::{histories, users};

use askama::Template;
use serde::Deserialize;

#[derive(Debug, Queryable)]
pub struct History {
    pub id: i32,
    pub input: String,
    pub done: bool,
}

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub pw: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub id: i32,
    pub email: String,
    pub pw: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "histories"]
pub struct AddHistory {
    pub input: String,
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
