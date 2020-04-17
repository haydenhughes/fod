use serde::Serialize;

#[derive(Serialize)]
pub struct IndexContext {
    items: Vec<String>,
}

impl IndexContext {
    pub fn new(items: Vec<String>) -> Self {
        IndexContext { items }
    }
}
