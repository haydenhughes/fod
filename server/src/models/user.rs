use super::All;
use crate::schema::users;
use crate::FodmapDbConn;
use argon2::{self, Config};
use diesel::dsl::{Eq, Filter};
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use getrandom::getrandom;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

type WithID<'a> = Eq<users::id, &'a i32>;
type ByID<'a> = Filter<All<users::table>, WithID<'a>>;

type WithName<'a> = Eq<users::name, &'a str>;
type ByName<'a> = Filter<All<users::table>, WithName<'a>>;

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub password: String,
}

impl NewUser {
    pub fn hash(self) -> Self {
        NewUser {
            name: self.name,
            password: {
                let salt = {
                    let mut s = [0u8; 16];
                    getrandom(&mut s).map(|_| s)
                }
                .expect("Unable to get random data for salt");
                argon2::hash_encoded(self.password.as_bytes(), &salt, &Config::default())
                    .expect("Unable to hash password")
            },
        }
    }
}

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
}

impl User {
    pub fn all() -> All<users::table> {
        users::table.select(users::all_columns)
    }

    pub fn with_id(id: &i32) -> WithID {
        users::id.eq(id)
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }

    pub fn with_name(name: &str) -> WithName {
        users::name.eq(name)
    }

    pub fn by_name(name: &str) -> ByName {
        Self::all().filter(Self::with_name(name))
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
                    .get_private("user_id")
                    .and_then(|cookie| cookie.value().parse::<i32>().ok())
                    .as_ref()
                    .and_then(|id| Self::by_id(id).first(&*conn).ok())
                    .map(|u| Outcome::Success(u))
                    .unwrap_or(Outcome::Failure((Status::Unauthorized, ())))
            })
            .expect("Error conneting to database")
    }
}
