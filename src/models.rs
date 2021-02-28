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

// trait Test {
//     fn request<T>(&self);

//     fn get_username<T, S>(&self, key: T, value: S) -> String {
//         let uservalue = key
//             .match_info()
//             .get(&self)
//             .expect("Failed to load user information.");
//         let user = uservalue.to_string();
//         user
//     }
// }

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

// fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
//     (t, s)
// }

// fn main() {
//     let t1 = make_tuple(1, 2);
//     let t2 = make_tuple("fjei", "jfie");
//     let t3 = make_tuple(vec![1, 2, 3], vec![4, 5, 6]);
// }
