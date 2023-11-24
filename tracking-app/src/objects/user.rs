use rocket::data::FromDataSimple;
use serde::{Deserialize, Serialize};

use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

impl FromDataSimple for User{
    type Error = String;

    fn from_data(req: &rocket::Request, data: rocket::Data) -> rocket::data::Outcome<Self, Self::Error> {
        let mut string = String::new();
        if let Err(e) = data.open().take(1024).read_to_string(&mut string){
            return rocket::Outcome::Failure((rocket::http::Status::InternalServerError, format!("{:?}", e)));
        }
        match serde_json::from_str::<User>(&string){
            Ok(user) => rocket::Outcome::Success(user),
            Err(e) => rocket::Outcome::Failure((rocket::http::Status::InternalServerError, format!("{:?}", e)))
        }
    }
}