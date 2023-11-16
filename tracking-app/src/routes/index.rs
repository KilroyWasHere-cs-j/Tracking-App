use rocket::State;
use crate::DB;

#[get("/")]
pub fn index(state: State<DB>) -> String {
    let mut db = state.db.lock().unwrap();
    format!("Current running database ID: {} Name: {}", db.get_id(), db.get_name())
}
