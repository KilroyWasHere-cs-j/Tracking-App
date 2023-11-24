use rocket::State;
use crate::DB;
use crate::objects;

// TODO - Add password hash checking
// TODO - Add regex search
#[get("/records/users/<username>")]
pub fn get_users(username: String, state: State<DB>) -> String{
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
pub fn get_records(username: String, state: State<DB>) -> String{
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
pub fn create_user(user: objects::user::User, state: State<DB>) -> String{
    let mut db = state.db.lock().unwrap();
    println!("Creating user: {:?}", user);
    format!("Created user: {:?}", user)
}

// #[post("/records/record/create", data = "<record>")]
// pub fn create_record(record: String, _state: State<DB>) -> String{
//     format!("Created record: {}", record)
// }
