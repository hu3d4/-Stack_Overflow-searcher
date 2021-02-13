use crate::errors::AppError;
use crate::models::*;
use crate::schema::*;

use diesel::pg::PgConnection;
use diesel::prelude::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn add_history(input: &AddHistory) -> Result<History, AppError> {
    let conn = establish_connection();
    return diesel::insert_into(histories::table)
        .values(input)
        .get_result(&conn)
        .map_err(|e| (AppError::DbError(e)));
}

pub fn add_user(username: &AddUser) -> Result<User, AppError> {
    let conn = establish_connection();
    return diesel::insert_into(users::table)
        .values(username)
        .get_result(&conn)
        .map_err(|e| (AppError::DbError(e)));
}

pub fn show_history() -> Result<Vec<History>, AppError> {
    use crate::diesel::prelude::*;
    use crate::schema::histories::dsl::*;
    let conn = establish_connection();
    return histories
        .filter(done.eq(true))
        .limit(15)
        .load::<History>(&conn)
        .map_err(|e| (AppError::DbError(e)));
}

// pub fn show_user() -> Result<Vec<User>, AppError> {
pub fn show_user() -> Vec<User> {
    use crate::diesel::prelude::*;
    use crate::schema::users::dsl::*;
    let conn = establish_connection();
    return users
        .filter(login_status.eq(true))
        .limit(15)
        .load::<User>(&conn)
        .unwrap();
    // .map_err(|e| (AppError::DbError(e)));
}

pub fn delete_all_history() -> Result<usize, AppError> {
    let conn = establish_connection();
    return diesel::delete(histories::table.filter(histories::id.gt(0)))
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

pub fn delete_single_user(id: i32) -> Result<usize, AppError> {
    use crate::schema::users::dsl;
    let delete_entory = DeleteHistory { id };
    let result = &delete_entory.id;
    let conn = establish_connection();
    return diesel::delete(dsl::users.filter(dsl::userid.eq(result)))
        .execute(&conn)
        .map_err(|e| (AppError::DbError(e)));
}
