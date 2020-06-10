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
pub struct CreateEntry {
    pub meal_type: String,
    pub timestamp: NaiveDateTime,
    pub comments: Option<String>
}
