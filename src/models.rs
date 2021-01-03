use askama::Template;
use serde::Serialize;

#[derive(Serialize)]
pub struct LogEntry {
    pub id: u32,
    pub input: String,
    pub done: bool,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub entries: Vec<LogEntry>,
}
