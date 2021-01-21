use crate::schema::history;

use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct History {
    pub id: i32,
    pub input: String,
    pub done: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name = "history"]
pub struct AddHistory {
    pub input: String,
}

#[derive(Template)]
#[template(path = "search-form.html")]
pub struct IndexTemplate {
    pub entries: Vec<History>,
}
