extern crate diesel;
extern crate diesel_demo;

use self::diesel_demo::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("Plz type your new user name here:");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let user_name = &name[..(name.len() - 1)]; // Drop the newline character

    let user = create_user(&connection, user_name, true);
    // println!("\nSaved draft {} with id {}", name, user.id);
    println!("\nSaved users: {:#?}", user);
}
