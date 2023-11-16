#![feature(proc_macro_hygiene, decl_macro)]

mod database;
mod routes;
mod structs;
mod daemon;

#[macro_use]
extern crate rocket;

use std::sync::Mutex;
use rocket::State;

pub struct DB{
    db: Mutex<database::db::Database>
}

fn main() {

    // Create a new database
    let mut tracking_app_db = database::db::Database::new(String::from("tracking_app_db"), 0);

    // Load the database from the file
    match tracking_app_db.load("tracking_app_db".to_string()){
        Ok(_) => println!("Database loaded successfully"),
        Err(_) => {
            println!("Database not found, creating a new one");
            tracking_app_db.save();
        }
    }

    // Create table for users and records
    let mut users = database::table::Table::new(String::from("users"), 0);
    let mut records = database::table::Table::new(String::from("records"), 1);

    // Add tables into the database
    tracking_app_db.insert_table(users);
    tracking_app_db.insert_table(records);

    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::index::index,
                routes::records::getRecords,
                routes::test::test,
                //routes::records::setRecord,
            ],
        )
        .manage(DB{db: Mutex::new(tracking_app_db)})
        .launch();
}
