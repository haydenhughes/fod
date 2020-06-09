use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Session<'a> {
    pub name: &'a str,
    pub password: &'a str,
}
