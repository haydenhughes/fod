pub mod errors;
mod context;

use crate::auth::User;
use crate::FodMapDatabase;
use context::IndexContext;
use diesel::prelude::*;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(user: User, conn: FodMapDatabase) -> Result<Template, diesel::result::Error> {
    Ok(Template::render("index", IndexContext::new(vec!(String::new()))))
}
