use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateEntry {
    pub meal_type: String,
    pub timestamp: NaiveDateTime,
    pub comments: Option<String>
}
