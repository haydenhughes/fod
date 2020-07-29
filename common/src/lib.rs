//! # FodMap Common
//!
//! A collection of shared data types between `fodmap_server` and `fodmap_client` in order to
//! to provide a consistant way of sharing data between the server and client.
//!
//! This is required as diesel won't compile for web assembly thus directly importing the database
//! models into `fodmap_client` is not feasible.

use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MealType {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateMealType {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Food {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateFood {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub id: i32,
    pub meal_type: MealType,
    pub timestamp: NaiveDateTime,
    pub hunger_before: i32,
    pub hunger_after: i32,
    pub foods: Vec<Food>,
    pub comments: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateEntry {
    pub meal_type: MealType,
    pub timestamp: NaiveDateTime,
    pub hunger_before: i32,
    pub hunger_after: i32,
    pub foods: Vec<Food>,
    pub comments: Option<String>
}
