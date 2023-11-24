use rocket::data::FromDataSimple;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task{
    pub id: i32,
    pub username: String,
    pub datetime: String,
    pub category: String,
    pub instance_counter: i32,
    pub start_time: String,
    pub end_time: String,
}

impl FromDataSimple for Task{
    type Error = String;

    fn from_data(req: &rocket::Request, data: rocket::Data) -> rocket::data::Outcome<Self, Self::Error> {
        let mut string = String::new();
        if let Err(e) = data.open().take(1024).read_to_string(&mut string){
            return rocket::Outcome::Failure((rocket::http::Status::InternalServerError, format!("{:?}", e)));
        }
        match serde_json::from_str::<Task>(&string){
            Ok(task) => rocket::Outcome::Success(task),
            Err(e) => rocket::Outcome::Failure((rocket::http::Status::InternalServerError, format!("{:?}", e)))
        }
    }
}