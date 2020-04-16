use serde::Serialize;

#[derive(Serialize)]
pub struct LoginContext {
    failed: bool,
    logout: bool
}

impl LoginContext {
    pub fn new(failed: bool, logout: bool) -> Self {
        LoginContext { failed, logout }
    }
}
