pub mod routes;

use crate::schema::users;
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use diesel::dsl::Eq;
use diesel::dsl::{Filter, Select};
use diesel::prelude::*;

type AllColumns = (users::id, users::username, users::password);

type All = Select<users::table, AllColumns>;
type WithName<'a> = Eq<users::username, &'a str>;
type ByUsername<'a> = Filter<All, WithName<'a>>;

#[derive(FromForm, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

impl NewUser {
    pub fn new(username: &str, password: &str) -> Result<Self, BcryptError> {
        hash(password, DEFAULT_COST).and_then(|p| {
            Ok(NewUser {
                username: username.into(),
                password: p,
            })
        })
    }
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn with_name(name: &str) -> WithName {
        users::username.eq(name)
    }

    pub fn all() -> All {
        users::table.select(users::all_columns)
    }

    pub fn by_username(name: &str) -> ByUsername {
        Self::all().filter(Self::with_name(name))
    }

    pub fn check_password<S: AsRef<str>>(&self, password: S) -> Result<bool, BcryptError> {
        verify(password.as_ref(), &self.password)
    }
}
