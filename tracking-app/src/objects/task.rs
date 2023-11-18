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