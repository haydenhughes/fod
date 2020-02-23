use crate::models::Item;
use serde::Serialize;

#[derive(Serialize)]
pub struct IndexContext {
    items: Vec<Item>,
}

impl IndexContext {
    pub fn new(items: Vec<Item>) -> Self {
        IndexContext { items }
    }
}
