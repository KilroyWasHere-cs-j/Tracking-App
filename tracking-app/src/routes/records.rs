use rocket::response::content::Json;
use rocket::State;
use crate::database::table::Table;
use crate::{DB, objects};
use serde_json::json;

// TODO - Add password hash checking
// TODO - Add regex search
#[get("/records/users/<username>")]
pub fn getUsers(username: String, state: State<DB>) -> String{
    let mut db = state.db.lock().unwrap();
    match db.get_table_mut(0){
        None => {
            format!("{}", "NULL")
        }
        Some(table) => {
            match table.query_by_value(username){
                None => {
                    format!("{}", "NULL")
                }
                Some(user) => {
                    format!("{:?}", user)
                }
            }
        }
    }
}

#[get("/records/records/<username>")]
pub fn getRecords(username: String, state: State<DB>) -> String{
    let mut db = state.db.lock().unwrap();
    match db.get_table_mut(1){
        None => {
            format!("{}", "NULL")
        }
        Some(table) => {
            match table.query_by_value(username){
                None => {
                    format!("{}", "NULL")
                }
                Some(record) => {
                    format!("{:?}", record)
                }
            }
        }
    }
}

// curl -X POST http://localhost:8000/records/user/create -d '{"id": 0, "username" : "John", "password_hash" : "sdapoios"}'

// TODO is any of this throws an error, it will literally break the entire server
#[post("/records/user/create", data = "<user>")]
pub fn createUser(user: String, state: State<DB>) -> String{
    let mut db = state.db.lock().unwrap();
    match db.get_table_mut(0){
        None => {
            format!("{}", "NULL")
        }
        Some(table) => {
            let raw_json = serde_json::from_str::<serde_json::Value>(&user).unwrap();
            println!("{:?}", raw_json);
            let user = objects::user::User{
                id: raw_json["id"].as_i64().unwrap() as i32,
                username: raw_json["username"].as_str().unwrap().to_string(),
                password_hash: raw_json["password_hash"].as_str().unwrap().to_string(),
            };
            table.insert(0, serde_json::to_string_pretty(&user).unwrap());
            db.save();
            format!("{}", "User created")
        }
    }
}

#[post("/records/record/create", data = "<record>")]
pub fn createRecord(record: String, state: State<DB>) -> String{
    format!("Created record: {}", record)
}
