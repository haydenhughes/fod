pub mod models;
pub mod routes;

use crate::schema::users;
use bcrypt::{hash, DEFAULT_COST, BcryptError};

#[derive(FromForm, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

impl NewUser {
    pub fn new(username: &str, password: &str) -> Result<Self, BcryptError> {
        hash(password, DEFAULT_COST).and_then(|p| {
            Ok(NewUser {
                username: username.into(),
                password: p,
            })
        })
    }
}
