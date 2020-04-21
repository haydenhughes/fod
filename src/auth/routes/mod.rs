mod context;

use super::NewUser;
use super::User;
use crate::FodMapDatabase;
use context::LoginContext;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[get("/login?<failed>&<logout>")]
pub fn login(failed: Option<bool>, logout: Option<bool>) -> Template {
    let context = LoginContext::new(failed.unwrap_or(false), logout.unwrap_or(false));
    Template::render("login", &context)
}

#[post("/login", data = "<request>")]
pub fn user_login(
    conn: FodMapDatabase,
    mut cookies: Cookies,
    request: Form<NewUser>,
) -> Result<Redirect, Status> {
    let user: Result<User, diesel::result::Error> =
        User::by_username(request.name.as_str()).first(&conn.0);

    match user {
        Ok(user) => match user.check_password(&request.password) {
            Ok(b) => {
                if b {
                    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
                    return Ok(Redirect::to(uri!(crate::routes::index)));
                } else {
                    return Ok(Redirect::to(uri!(login: failed = true, logout = false)));
                }
            }
            // FIXME: Actually return a helpful error
            Err(_) => Err(Status::InternalServerError),
        },
        Err(diesel::result::Error::NotFound) => Ok(Redirect::to(uri!(login: failed = true, logout = false))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/logout")]
pub fn user_logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to(uri!(login: failed = false, logout = true))
}
