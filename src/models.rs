use crate::schema::history;

use askama::Template;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct History {
    pub id: u32,
    pub input: String,
    pub done: bool,
}

#[derive(Insertable)]
#[table_name = "history"]
pub struct HistoryEntry {
    pub input: String,
    pub done: bool,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub entries: Vec<History>,
}
