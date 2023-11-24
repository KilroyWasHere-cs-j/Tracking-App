#![feature(proc_macro_hygiene, decl_macro)]

mod database;
mod routes;

mod objects;

#[macro_use]
extern crate rocket;

use std::sync::Mutex;

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up.\
    If the problem persists, please contact the developer at gmtower1@gmail.com"
}

#[catch(404)]
fn not_found() -> String{
    let message = r#"
    Valid end points:

    Index:

    / - Index

    GET:

    /records/users/<username>/<passwordhash> - Gets a user from the database
    /records/records/<username>/<passwordhash> - Gets a users records for the database

    POST:

    /records/user/create - Creates a user in the database
    /records/record/create - Creates a record in the database

    Any questions please contact the developer at gmtower1@gmail.com
    "#;
    format!("{}", message)
}

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
    let users = database::table::Table::new(String::from("users"), 0);
    let records = database::table::Table::new(String::from("records"), 1);
    //
    // // Add tables into the database
    tracking_app_db.insert_table(users);
    tracking_app_db.insert_table(records);

    tracking_app_db.save();

    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::index::index,
                routes::records::get_records,
                routes::records::get_users,
                routes::test::test,
                routes::records::create_user,
                //routes::records::create_record,
                routes::dev::print_db,
                routes::dev::db_info,
            ],
        )
        .manage(DB{db: Mutex::new(tracking_app_db)})
        .register(catchers![not_found, internal_error])
        .launch();
}
