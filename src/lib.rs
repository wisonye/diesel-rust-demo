#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use models::{NewUser, User};
use std::env;

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(conn: &PgConnection, name: &'a str, is_male: bool) -> User {
// pub fn create_user<'a>(conn: &PgConnection, name: &'a str, is_male: bool) -> usize {
    use schema::users;

    let new_user = NewUser {
        id: None,
        name: name,
        is_male: is_male
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        // .execute(conn)
        .expect("Error saving new post")
}
