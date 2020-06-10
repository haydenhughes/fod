use crate::models::{NewUser, User};
use crate::schema;
use crate::FodmapDbConn;
use diesel::prelude::*;
use fodmap_common::Session;
use rocket::http::Status;
use rocket::http::{Cookie, Cookies};
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/sessions..")]
pub fn get_sessions(_user: User) -> Status {
    Status::MethodNotAllowed
}

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
                    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
                    status::Accepted(Some("Logged in successfully"))
                })
                .ok_or(status::Unauthorized(Some("Invalid user name or password")))
        })
}

#[put("/sessions..")]
pub fn put_sessions(_user: User) -> Status {
    Status::MethodNotAllowed
}

#[delete("/sessions")]
pub fn delete_session(mut cookies: Cookies) -> status::NoContent {
    cookies.remove(Cookie::named("user_id"));
    status::NoContent
}

#[get("/users/<id>")]
pub fn get_user(
    _user: User,
    conn: FodmapDbConn,
    id: Option<i32>,
) -> Result<Json<Vec<User>>, status::NotFound<&'static str>> {
    match id {
        Some(id) => User::by_id(&id)
            .get_results::<User>(&*conn)
            .map(Json)
            .map_err(|e| {
                warn!("Unable to query user: {}", e);
                status::NotFound("The requested user does not exist")
            }),
        None => Ok(User::all()
            .get_results::<User>(&*conn)
            .map(Json)
            .expect("Unable to query users")),
    }
}

#[post("/users", data = "<content>")]
pub fn create_user(
    _user: User,
    conn: FodmapDbConn,
    content: Json<Session>,
) -> Result<status::Created<Json<User>>, status::Conflict<&'static str>> {
    diesel::insert_into(schema::users::table)
        .values(NewUser::from(content.into_inner()))
        .get_result::<User>(&*conn)
        .map(Json)
        .map(|r| status::Created(uri!(get_user: r.id).to_string(), Some(r)))
        .map_err(|e| {
            warn!("Unable to create user: {}", e);
            status::Conflict(Some("User already exists"))
        })
}

#[put("/users/<id>", data = "<content>")]
pub fn update_user(
    _user: User,
    conn: FodmapDbConn,
    id: i32,
    content: Json<Session>,
) -> Result<Json<User>, status::NotFound<&'static str>> {
    User::by_id(&id)
        .get_result::<User>(&*conn)
        .map(|user| {
            diesel::update(&user)
                .set(NewUser::from(content.into_inner()))
                .get_result::<User>(&*conn)
                .map(Json)
                .expect("Unable to update user")
        })
        .map_err(|e| {
            warn!("Unable to query user: {}", e);
            status::NotFound("The user your trying to update doesn't exist")
        })
}

#[delete("/users/<id>")]
pub fn delete_user(_user: User, conn: FodmapDbConn, id: i32) -> status::NoContent {
    match User::by_id(&id).get_result::<User>(&*conn) {
        Ok(u) => diesel::delete(&u)
            .execute(&*conn)
            .map(|_| status::NoContent)
            .expect("Unable to delete user"),
        Err(e) => {
            warn!("Unable to query user: {}", e);
            status::NoContent
        }
    }
}
