use crate::models::{AddHistory, DeleteHistory, History};
use crate::schema::history;
use crate::schema::history::dsl;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn add_history(input: String) -> History {
    let history_entory = AddHistory { input };
    let conn = establish_connection();
    diesel::insert_into(dsl::history)
        .values(&history_entory)
        .get_result(&conn)
        .expect("Error saving new post")
}

pub fn show_history() -> Vec<History> {
    let conn = establish_connection();
    return dsl::history
        .filter(dsl::done.eq(true))
        .limit(15)
        .load::<History>(&conn)
        .expect("Error loading posts");
}

pub fn delete_history() {
    let conn = establish_connection();
    diesel::delete(history::table.filter(history::id.gt(0)))
        .execute(&conn)
        .expect("Failed to clean up history");
}

pub fn delete_one_history(id: i32) {
    let delete_entory = DeleteHistory { id };
    let result = &delete_entory.id;
    let conn = establish_connection();
    diesel::delete(dsl::history.filter(dsl::id.eq(result)))
        .execute(&conn)
        .expect("msg: &str");
}
