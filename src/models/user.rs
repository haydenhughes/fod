use crate::schema::users;
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;

type AllColumns = (users::id, users::username, users::password);

type All = Select<users::table, AllColumns>;

type WithID<'a> = Eq<users::id, &'a i32>;
type ByID<'a> = Filter<All, WithID<'a>>;

type WithUserName<'a> = Eq<users::username, &'a str>;
type ByUserName<'a> = Filter<All, WithUserName<'a>>;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    username: String,
    password: String,
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
    pub fn with_id(id: &i32) -> WithID {
        users::id.eq(id)
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

    pub fn check_password<S: AsRef<str>>(&self, password: S) -> Result<bool, BcryptError> {
        verify(password.as_ref(), &self.password)
    }
}
