use super::FodmapDbConn;
use crate::schema::users;
use argon2::{self, Config};
use fodmap_common::Session;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use getrandom::getrandom;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

type AllColumns = (users::id, users::user_name, users::password);

type All = Select<users::table, AllColumns>;

type WithID<'a> = Eq<users::id, &'a i32>;
type ByID<'a> = Filter<All, WithID<'a>>;

type WithName<'a> = Eq<users::user_name, &'a str>;
type ByName<'a> = Filter<All, WithName<'a>>;

#[derive(Debug)]
pub enum AuthenticationError {
    DBError,
    Invalid,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    user_name: String,
    password: String,
}

impl From<Session> for NewUser {
    fn from(s: Session) -> Self {
        NewUser {
            user_name: s.user_name,
            password: {
                let salt = {
                    let mut s = [0u8; 16];
                    getrandom(&mut s).map(|_| s)
                }
                .expect("Unable to get random data for salt");
                argon2::hash_encoded(s.password.as_bytes(), &salt, &Config::default())
                    .expect("Unable to hash password")
            },
        }
    }
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub password: String,
}

impl User {
    pub fn with_id(id: &i32) -> WithID {
        users::id.eq(id)
    }

    pub fn with_user_name(name: &str) -> WithName {
        users::user_name.eq(name)
    }

    pub fn all() -> All {
        users::table.select(users::all_columns)
    }

    pub fn by_user_name(name: &str) -> ByName {
        Self::all().filter(Self::with_user_name(name))
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }

    pub fn is_password<S: AsRef<[u8]>>(&self, password: S) -> bool {
        argon2::verify_encoded(&self.password, password.as_ref())
            .expect("Unable to verify password")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        request
            .guard::<FodmapDbConn>()
            .succeeded()
            .map(|conn| {
                request
                    .cookies()
                    .get_private("session_id")
                    .and_then(|cookie| cookie.value().parse::<i32>().ok())
                    .as_ref()
                    .and_then(|id| Self::by_id(id).first(&*conn).ok())
                    .map(|u| Outcome::Success(u))
                    .unwrap_or(Outcome::Failure((Status::Unauthorized, ())))
            })
            .expect("Error conneting to database")
    }
}
