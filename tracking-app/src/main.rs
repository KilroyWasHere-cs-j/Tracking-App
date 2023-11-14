#![feature(proc_macro_hygiene, decl_macro)]

mod routes;

#[macro_use] extern crate rocket;

fn main() {
    rocket::ignite().mount("/", routes![routes::index::hello, routes::records::getRecords]).launch();
}