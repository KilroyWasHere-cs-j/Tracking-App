#![feature(proc_macro_hygiene, decl_macro)]

mod database;
mod routes;
mod structs;

#[macro_use]
extern crate rocket;

use std::sync::Mutex;
use rocket::State;

pub struct DB{
    db: Mutex<database::db::Database>
}

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

    rocket::ignite()
        .mount(z
            "/",
            routes![
                routes::index::index,
                routes::records::getRecords,
                routes::test::test,
                routes::records::setRecord,
            ],
        )
        .manage(DB{db: Mutex::new(tracking_app_db)})
        .launch();
}
