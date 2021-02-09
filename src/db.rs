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

pub fn add_history(input: String) -> Result<History, AppError> {
    let history_entory = AddHistory { input };
    let conn = establish_connection();
    return diesel::insert_into(histories::table)
        .values(&history_entory)
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

pub fn delete_all_history() -> Result<usize, AppError> {
    let conn = establish_connection();
    return diesel::delete(histories::table.filter(histories::id.gt(0)))
        .execute(&conn)
        .map_err(|e| (AppError::DbError(e)));
}

pub fn delete_one_history(id: i32) -> Result<usize, AppError> {
    use crate::schema::histories::dsl;
    let delete_entory = DeleteHistory { id };
    let result = &delete_entory.id;
    let conn = establish_connection();
    return diesel::delete(dsl::histories.filter(dsl::id.eq(result)))
        .execute(&conn)
        .map_err(|e| (AppError::DbError(e)));
}

pub fn add_user(user: &NewUser) -> std::result::Result<User, AppError> {
    let conn = establish_connection();
    return diesel::insert_into(users::table)
        .values(user)
        .get_result(&conn)
        .map_err(|e| (AppError::DbError(e)));
}

#[cfg(test)]
mod tests {
    use super::{Connection, PgConnection, RunQueryDsl};
    use crate::db::add_history;
    use pretty_assertions::assert_eq;

    #[test]
    fn add_history_test() {
        let conn = PgConnection::establish(&format!(
            "postgres://so_searcher:so_searcher_password@0.0.0.0:5433/so_searcher"
        ))
        .unwrap();

        diesel::sql_query("INSERT INTO history (input) VALUES ('text')")
            .execute(&conn)
            .unwrap();
        let u = add_history("text".to_string()).unwrap();

        assert_eq!(u.input, "text".to_string());
    }
}
