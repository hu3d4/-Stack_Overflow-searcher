use crate::schema::*;

use askama::Template;
use serde::Deserialize;

#[derive(Debug, Queryable)]
pub struct History {
    pub id: i32,
    pub input: String,
    pub done: bool,
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
