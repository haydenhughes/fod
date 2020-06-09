use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
}
