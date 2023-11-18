use rocket::State;
use crate::database::table::Table;
use crate::DB;

// TODO - Add password hash checking
// TODO - Add regex search
#[get("/records/<username>/<passwordhash>")]
pub fn getRecords(username: String, passwordhash: String, state: State<DB>) -> String{
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

// #[post("/records/<username>/<passwordhash>/set", format = "json", data = "<record>")]
// pub fn setRecord(username: String, passwordhash: String, record: String, state: State<DB>) -> String{
//     format!("Hello, {}! Your password hash is {}. You want to set a record of type {} with data", username, passwordhash, record)
// }
