use crate::schema::{histories, users};

use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable)]
pub struct History {
    pub id: i32,
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
#[table_name = "users"]
pub struct AddUser {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub username: String,
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

#[derive(Template)]
#[template(path = "register/result.html")]
pub struct ResultTemplate {
    pub entries: Vec<History>,
}
