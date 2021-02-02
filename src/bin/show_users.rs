extern crate diesel;
extern crate diesel_demo;

use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .filter(is_male.eq(true))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:#?}", &user);
        println!("-----------\n");
    }
}
