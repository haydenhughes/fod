//! We can't import them straight from server because diesel won't build for webasm.

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Session<'a> {
    pub name: &'a str,
    pub password: &'a str,
}
