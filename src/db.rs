use crate::errors::AppError;
use crate::models::{AddHistory, History};
use crate::schema::history;

use diesel::pg::PgConnection;
use diesel::prelude::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
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

pub fn show_history() -> Result<Vec<History>, AppError> {
    use crate::diesel::prelude::*;
    use crate::schema::history::dsl::*;
    let conn = establish_connection();
    return history
        .filter(done.eq(true))
        .limit(15)
        .load::<History>(&conn)
        .map_err(|e| (AppError::DbError(e)));
}

pub fn delete_history() -> Result<usize, AppError> {
    let conn = establish_connection();
    return diesel::delete(history::table.filter(history::id.gt(0)))
        .execute(&conn)
        .map_err(|e| (AppError::DbError(e)));
}
