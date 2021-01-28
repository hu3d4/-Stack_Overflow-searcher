use crate::errors::AppError;
use crate::models::{AddHistory, History};
use crate::schema::*;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn add_history(input: String) -> Result<History, AppError> {
    let history_entory = AddHistory { input };
    let conn = establish_connection();
    return diesel::insert_into(history::table)
        .values(&history_entory)
        .get_result(&conn)
        .map_err(|e| (AppError::DbError(e)));
}

pub fn show_history() -> Vec<History> {
    use crate::diesel::prelude::*;
    use crate::schema::history::dsl::*;
    let conn = establish_connection();
    return history
        .filter(done.eq(true))
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
