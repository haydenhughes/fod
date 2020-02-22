use serde::Serialize;

#[derive(Serialize)]
pub struct LoginContext {
    failed: bool,
}

impl LoginContext {
    pub fn new(failed: bool) -> Self {
        LoginContext { failed }
    }
}
