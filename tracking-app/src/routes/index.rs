use rocket::State;
use serde::{Deserialize, Serialize};
use crate::DB;

#[derive(Debug, Deserialize, Serialize)]
struct DbInfo {
    db_id: u32,
    db_name: String,
}

#[get("/")]
pub fn index(state: State<DB>) -> String {
    let db = state.db.lock().unwrap();
    let message = DbInfo {
        db_id: db.get_id() as u32,
        db_name: db.get_name().to_string(),
    };
    format!("{}", serde_json::to_string(&message).unwrap())
}
