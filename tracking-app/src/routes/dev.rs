use rocket::State;
use crate::DB;

#[get("/dev/print_db")]
pub fn print_db(state: State<DB>) -> String{
    let db = state.db.lock().unwrap();
    format!("{:?}", db)
}

#[get("/dev/db_info")]
pub fn db_info(state: State<DB>) -> String{
    let db = state.db.lock().unwrap();
    format!("{:?}, {:?}", db.get_name(), db.get_id())
}