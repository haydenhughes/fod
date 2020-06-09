use crate::FodmapDbConn;
use crate::models::{Session, NewUser, User};
use crate::schema;
use diesel::prelude::*;
use rocket::http::{Cookie, Cookies};
use rocket::response::status;
use rocket_contrib::json::Json;

#[post("/sessions", data = "<session>")]
pub fn create_session(
    conn: FodmapDbConn,
    mut cookies: Cookies,
    session: Json<Session>,
) -> Result<status::Accepted<&'static str>, status::Unauthorized<&'static str>> {
    User::by_name(&session.name)
        .get_result::<User>(&*conn)
        .map_err(|e| {
            warn!("Unable to authenticate user: {}", e);
            status::Unauthorized(Some("Invalid user name or password"))
        })
        .and_then(|user| {
            user.is_password(&session.password)
                .then_some({
                    cookies.add_private(Cookie::new("session_id", user.id.to_string()));
                    status::Accepted(Some("Logged in successfully"))
                })
                .ok_or(status::Unauthorized(Some("Invalid user name or password")))
        })
}

#[delete("/sessions")]
pub fn delete_session(mut cookies: Cookies) -> status::NoContent {
    cookies.remove(Cookie::named("session_id"));
    status::NoContent
}

#[post("/users", data = "<user>")]
pub fn create_user(
    conn: FodmapDbConn,
    user: Json<NewUser>,
) -> Result<status::NoContent, status::Conflict<&'static str>> {
    diesel::insert_into(schema::users::table)
        .values(user.into_inner())
        .execute(&*conn)
        .map(|_| status::NoContent)
        .map_err(|e| {
            warn!("Unable to create user: {}", e);
            status::Conflict(Some("User already exists"))
        })
}
