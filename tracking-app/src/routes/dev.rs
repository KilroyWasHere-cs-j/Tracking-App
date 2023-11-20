use rocket::State;
use crate::DB;

#[get("/dev/print_db")]
pub fn print_db(state: State<DB>) -> String{
    let db = state.db.lock().unwrap();
    format!("{:?}", db)
}