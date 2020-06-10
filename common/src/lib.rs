//! # FodMap Common
//!
//! A collection of shared data types between `fodmap_server` and `fodmap_client` in order to
//! to provide a consistant way of sharing data between the server and client.
//!
//! This is required as diesel won't compile for web assembly thus directly importing the database
//! models into `fodmap_client` is not feasible.

use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct MealType {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateMealType {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Food {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateFood {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub id: i32,
    pub meal_type: MealType,
    pub timestamp: NaiveDateTime,
    pub foods: Vec<Food>,
    pub comments: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateEntry {
    pub meal_type: MealType,
    pub timestamp: NaiveDateTime,
    pub foods: Vec<Food>,
    pub comments: Option<String>
}
