pub mod routes;

use crate::schema::users;
use crate::FodMapDatabase;
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use rocket::request::{FromRequest, Outcome, Request};

type AllColumns = (users::userid, users::username, users::password);

type All = Select<users::table, AllColumns>;

type WithID<'a> = Eq<users::userid, &'a i32>;
type ByID<'a> = Filter<All, WithID<'a>>;

type WithUserName<'a> = Eq<users::username, &'a str>;
type ByUserName<'a> = Filter<All, WithUserName<'a>>;

#[derive(FromForm, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

impl NewUser {
    pub fn new(username: &str, password: &str) -> Result<Self, BcryptError> {
        User::hash_password(password).and_then(|p| {
            Ok(NewUser {
                username: username.into(),
                password: p,
            })
        })
    }
}

#[derive(Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn with_id(id: &i32) -> WithID {
        users::userid.eq(id)
    }

    pub fn with_username(name: &str) -> WithUserName {
        users::username.eq(name)
    }

    pub fn all() -> All {
        users::table.select(users::all_columns)
    }

    pub fn by_username(name: &str) -> ByUserName {
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
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let conn = request.guard::<FodMapDatabase>()?;
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse::<i32>().ok())
            .as_ref()
            .and_then(|id| Self::by_id(id).first(&conn.0).ok())
            .map(|u| Outcome::Success(u))
            .unwrap_or(Outcome::Forward(()))
    }
}
