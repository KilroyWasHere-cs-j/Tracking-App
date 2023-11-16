use rocket::State;
use crate::DB;

#[get("/records/<username>/<passwordhash>")]
pub fn getRecords(username: String, passwordhash: String, state: State<DB>) -> String{
    let mut db = state.db.lock().unwrap();
    format!("Hello, {}! Your password hash is {}", username, passwordhash)
}

#[get("/records/<username>/<passwordhash>/recordtype/<recordtype>")]
pub fn getRecordsByType(username: String, passwordhash: String, recordtype: String, state: State<DB>) -> String{
    format!("Hello, {}! Your password hash is {}. You want records of type {}", username, passwordhash, recordtype)
}

// #[post("/records/<username>/<passwordhash>/set", format = "json", data = "<record>")]
// pub fn setRecord(username: String, passwordhash: String, record: String, state: State<DB>) -> String{
//     format!("Hello, {}! Your password hash is {}. You want to set a record of type {} with data", username, passwordhash, record)
// }
