mod context;

use super::models::User;
use super::NewUser;
use crate::FodMapDatabase;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use context::LoginContext;

#[get("/login?<failed>")]
pub fn login(failed: Option<bool>) -> Template {
    let context = LoginContext::new(failed.unwrap_or(false));
    Template::render("login", &context)
}

#[post("/login", data = "<request>")]
pub fn user_login(conn: FodMapDatabase, request: Form<NewUser>) -> Result<Redirect, Status> {
    let user: Result<User, diesel::result::Error> =
        User::by_username(request.username.as_str()).first(&conn.0);

    match user {
        Ok(user) => match user.check_password(&request.password) {
            Ok(b) => {
                if b {
                    return Ok(Redirect::to(uri!(crate::routes::index)));
                } else {
                    return Ok(Redirect::to(uri!(login: failed = true)));
                }
            }
            // FIXME: Actually return a helpful error
            Err(_) => Err(Status::InternalServerError),
        },
        Err(diesel::result::Error::NotFound) => Ok(Redirect::to(uri!(login: failed = true))),
        Err(_) => Err(Status::InternalServerError),
    }
}
