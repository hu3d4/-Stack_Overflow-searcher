use crate::errors::AppError;
use crate::models::DeleteHistory;
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

pub fn delete_all_history() -> Result<usize, AppError> {
    let conn = establish_connection();
    return diesel::delete(history::table.filter(history::id.gt(0)))
        .execute(&conn)
        .map_err(|e| (AppError::DbError(e)));
}

pub fn delete_one_history(id: i32) -> Result<usize, AppError> {
    use crate::schema::history::dsl;
    let delete_entory = DeleteHistory { id };
    let result = &delete_entory.id;
    let conn = establish_connection();
    return diesel::delete(dsl::history.filter(dsl::id.eq(result)))
        .execute(&conn)
        .map_err(|e| (AppError::DbError(e)));
}

use crate::diesel::OptionalExtension;

impl History {
    pub fn get_input_by_user(conn: &PgConnection, inputs: String) -> Option<History> {
        use crate::schema::history::dsl::*;
        history
            .filter(input.eq(inputs))
            .first(conn)
            .optional()
            .unwrap()
    }
}

struct TestContext {}

// struct TestContext {
//     base_url: String,
//     db_name: String,
// }

impl TestContext {
    fn new() -> Self {
        // use crate::embedded_migrations;
        // use diesel_migrations::embed_migrations;
        embed_migrations!("migrations/");

        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // let conn = establish_connection();
        let conn = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        embedded_migrations::run(&conn);
        println!("Set up resources");
        Self {}
    }

    // fn new() -> Self {
    //     let conn = PgConnection::establish(&format!("{}/{}", base_url, db_name))
    //         .expect(&format!("Cannot connect to {} database", db_name));
    //     embedded_migrations::run(&conn);
    //     println!("Set up resources");
    //     Self {}
    // }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        println!("Clean up resources");
    }
}

struct TestContexts {
    base_url: String,
    db_name: String,
}

impl TestContexts {
    fn new(base_url: &str, db_name: &str) -> Self {
        // let postgres_url = format!("{}/so_searcher", base_url);
        // let conn = establish_connection();
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        // let conn =
        //     PgConnection::establish(&postgres_url).expect("Cannot connect to postgres database.");
        let query = diesel::sql_query(format!("CREATE DATABASE {}", db_name).as_str());
        query
            .execute(&conn)
            .expect(format!("Could not create database {}", db_name).as_str());
        Self {
            base_url: base_url.to_string(),
            db_name: db_name.to_string(),
        }
    }
}

impl Drop for TestContexts {
    fn drop(&mut self) {
        // let postgres_url = format!("{}/so_searcher", self.base_url);
        // let conn = establish_connection();
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        let disconnect_users = format!(
            "SELECT pg_terminate_backend(pid)
            FROM pg_stat_activity
            WHERE datname = '{}';",
            self.db_name
        );
        diesel::sql_query(disconnect_users.as_str())
            .execute(&conn)
            .unwrap();
        let query = diesel::sql_query(format!("DROP DATABASE {}", self.db_name).as_str());
        query
            .execute(&conn)
            .expect(&format!("Couldn't drop database {}", self.db_name));
    }
}

#[cfg(test)]
mod tests {
    use super::{Connection, History, PgConnection, RunQueryDsl, TestContext};
    // use crate::db::establish_connection;
    use pretty_assertions::assert_eq;
    #[test]
    fn try_it() {
        let _ctx = TestContext::new();
    }

    #[test]
    fn insert_user_test() {
        // let _ctx = setup_database(
        //     "postgres://so_searcher:so_searcher_password@0.0.0.0:5433",
        //     "so_searcher",
        // );
        // DATABASE_URL=postgres://so_searcher:so_searcher_password@0.0.0.0:5433/so_searcher
        // let conn = establish_connection();
        let conn = PgConnection::establish(&format!(
            "postgres://so_searcher:so_searcher_password@0.0.0.0:5433/so_searcher"
        ))
        .unwrap();

        // Now do your test.
        diesel::sql_query("INSERT INTO users (input) VALUES ('text')")
            .execute(&conn)
            .unwrap();
        let u = History::get_input_by_user(&conn, "text".to_string()).unwrap();

        assert_eq!(u.input, "text".to_string());
    }

    #[test]
    fn remove_user_test() {
        // let _ctx = setup_database(
        //     "postgres://so_searcher:so_searcher_password@0.0.0.0:5433",
        //     "so_searcher",
        // );
        // let conn = establish_connection();
        let conn = PgConnection::establish(&format!(
            "postgres://so_searcher:so_searcher_password@0.0.0.0:5433/so_searcher",
        ))
        .unwrap();
    }
}
// #[cfg(test)]
// mod tests {
//     use super::{
//         establish_connection, AddHistory, Connection, PgConnection, RunQueryDsl, TestContext,
//     };
//     use pretty_assertions::assert_eq;
//     #[test]
//     fn try_it() {
//         let _ctx = TestContext::new();
//     }
//     #[test]
//     fn insert_user_test() {
//         use diesel_migrations::setup_database;
//         let _ctx = setup_database(&establish_connection());

//         diesel::sql_query("INSERT INTO users (input) VALUES ('input text')")
//             .execute(&establish_connection())
//             .unwrap();
//         let u = AddHistory::get_input_by_user(&establish_connection(), "input text".to_string())
//             .unwrap();

//         assert_eq!(u.input, "input text".to_string());
//     }

//     #[test]
//     fn remove_user_test() {
//         use diesel_migrations::setup_database;

//         let _ctx = setup_database(&establish_connection());

//         let conn = PgConnection::establish(&format!(
//             "postgres://so_searcher:so_searcher_password@0.0.0.0:5433/so_searcher"
//         ))
//         .unwrap();
//     }
// }
