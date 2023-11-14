#![feature(proc_macro_hygiene, decl_macro)]

mod routes;
mod database;

#[macro_use] extern crate rocket;

use std::fs::File;

fn main() {
    let mut tracking_app_db = database::db::Database::new(String::from("tracking_app_db"), 0);

    println!("Database name {} is up.", tracking_app_db.get_name());

    tracking_app_db.load("tracking_app_db".to_string());

    let mut users = database::table::Table::new(String::from("users"), 0);
    let mut records = database::table::Table::new(String::from("records"), 1);

    users.insert(0, String::from("password"));
    users.insert(1, String::from("password"));

    tracking_app_db.insert_table(users);
    tracking_app_db.insert_table(records);

    tracking_app_db.save();

    rocket::ignite().mount("/", routes![routes::index::hello, routes::records::getRecords]).launch();
}