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

pub fn create_post<'a>(conn: &PgConnection, input: &'a str) -> History {
    let history_entory = AddHistory { input };
    diesel::insert_into(history::table)
        .values(&history_entory)
        .get_result(conn)
        .expect("Error saving new post")
}
// pub async fn create_post<'a>(conn: &PgConnection, input: &'a str) -> History {
//     let history_entory = AddHistory { input };
//     diesel::insert_into(history::table)
//         .values(&history_entory)
//         .get_result(conn)
//         .expect("Error saving new post")
// }

pub fn show_history() -> Vec<History> {
    use crate::diesel::prelude::*;
    use crate::schema::history::dsl::*;
    let connection = establish_connection();
    return history
        .filter(done.eq(true))
        .limit(15)
        .load::<History>(&connection)
        .expect("Error loading posts");
}
