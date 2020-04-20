pub mod routes;

use crate::schema::users;
use crate::FodMapDatabase;
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::Serialize;

type AllColumns = (users::id, users::name, users::password);

type All = Select<users::table, AllColumns>;

type WithID<'a> = Eq<users::id, &'a i32>;
type ByID<'a> = Filter<All, WithID<'a>>;

type WithName<'a> = Eq<users::name, &'a str>;
type ByName<'a> = Filter<All, WithName<'a>>;

#[derive(Debug)]
pub enum AuthenticationError {
    DBError,
    Invalid,
}

#[derive(FromForm, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub password: String,
}

impl NewUser {
    pub fn new(username: &str, password: &str) -> Result<Self, BcryptError> {
        User::hash_password(password).and_then(|p| {
            Ok(NewUser {
                name: username.into(),
                password: p,
            })
        })
    }
}

#[derive(Queryable, Identifiable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
}

impl User {
    pub fn with_id(id: &i32) -> WithID {
        users::id.eq(id)
    }

    pub fn with_username(name: &str) -> WithName {
        users::name.eq(name)
    }

    pub fn all() -> All {
        users::table.select(users::all_columns)
    }

    pub fn by_username(name: &str) -> ByName {
        Self::all().filter(Self::with_username(name))
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }

    pub fn hash_password<S: AsRef<str>>(password: S) -> Result<String, BcryptError> {
        hash(password.as_ref(), DEFAULT_COST)
    }

    pub fn check_password<S: AsRef<str>>(&self, password: S) -> Result<bool, BcryptError> {
        verify(password.as_ref(), &self.password)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = AuthenticationError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        request
            .guard::<FodMapDatabase>()
            .map_failure(|_| (Status::InternalServerError, AuthenticationError::DBError))
            .and_then(|conn| {
                request
                    .cookies()
                    .get_private("user_id")
                    .and_then(|cookie| cookie.value().parse::<i32>().ok())
                    .as_ref()
                    .and_then(|id| Self::by_id(id).first(&conn.0).ok())
                    .map(|u| Outcome::Success(u))
                    .unwrap_or(Outcome::Failure((
                        Status::Unauthorized,
                        AuthenticationError::Invalid,
                    )))
            })
    }
}
