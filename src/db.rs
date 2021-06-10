use crate::errors::AppError;
use crate::models::{DeleteHistory, GetHistory, History};
use crate::schema::histories;

use diesel::pg::PgConnection;
use diesel::prelude::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_history(input: String, username: String) -> Result<History, AppError> {
    let history_entory = GetHistory { input, username };
    let conn = establish_connection();
    return diesel::insert_into(histories::table)
        .values(&history_entory)
        .get_result(&conn)
        .map_err(|e| (AppError::DbError(e)));
}

pub fn show_history() -> Result<Vec<History>, AppError> {
    use crate::schema::histories::dsl::*;
    let conn = establish_connection();
    return histories
        .filter(done.eq(true))
        .limit(15)
        .load::<History>(&conn)
        .map_err(|e| (AppError::DbError(e)));
}

pub fn delete_all_history(username: &String) -> Result<usize, AppError> {
    let conn = establish_connection();
    return diesel::delete(
        histories::table.filter(histories::username.eq(format!("{}", username))),
    )
    .execute(&conn)
    .map_err(|e| (AppError::DbError(e)));
}

pub fn delete_single_history(id: i32) -> Result<usize, AppError> {
    use crate::schema::histories::dsl;
    let delete_entory = DeleteHistory { id };
    let result = &delete_entory.id;
    let conn = establish_connection();
    return diesel::delete(dsl::histories.filter(dsl::id.eq(result)))
        .execute(&conn)
        .map_err(|e| (AppError::DbError(e)));
}
