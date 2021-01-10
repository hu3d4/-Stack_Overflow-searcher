use crate::schema::history;

use askama::Template;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct History {
    pub id: i32,
    pub input: String,
    pub done: bool,
}

#[derive(Insertable)]
#[table_name = "history"]
pub struct AddHistory<'a> {
    pub input: &'a str,
}

#[derive(Template)]
#[template(path = "search-form.html")]
pub struct IndexTemplate {
    pub entries: Vec<History>,
}
