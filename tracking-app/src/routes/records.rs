use rocket::State;
use crate::DB;
use crate::objects;
use crate::objects::task::Task;
use crate::objects::user::User;

// TODO - Add password hash checking
// TODO - Add regex search

// curl http://localhost:8000/records/users/<username>/<passwordhash>
#[get("/records/users/<username>/<passwordhash>")]
pub fn get_users(username: String, passwordhash:String, state: State<DB>) -> String{
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

// curl http://localhost:8000/records/records/<username>/<passwordhash>

#[get("/records/records/<username>/<passwordhash>")]
pub fn get_records(username: String, passwordhash:String, state: State<DB>) -> String{
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

// curl http://localhost:8000/records/users/John/sdapoios

#[post("/records/user/create", data = "<user>")]
pub fn create_user(user: User, state: State<DB>) -> String{
    let mut db = state.db.lock().unwrap();
    match db.get_table_mut(0){
        None => {
            format!("{}", "NULL")
        }
        Some(table) => {
            table.insert(table.get_latest_id(), serde_json::to_string(&user).unwrap());
            format!("{}", "Success")
        }
    }
}

// curl https://tracking-app-backend.onrender.com/records/record/John/sdapoios
#[post("/records/record/create", data = "<record>")]
pub fn create_record(record: Task, _state: State<DB>) -> String{
    let mut db = _state.db.lock().unwrap();
    match db.get_table_mut(1){
        None => {
            format!("{}", "NULL")
        }
        Some(table) => {
            table.insert(table.get_latest_id(), serde_json::to_string(&record).unwrap());
            format!("{}", "Success")
        }
    }
}
