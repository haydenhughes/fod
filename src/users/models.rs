use crate::schema::users;
use diesel::dsl::Eq;
use bcrypt::{verify, BcryptError};
use diesel::prelude::*;
use diesel::dsl::{Select, Filter};

type AllColumns = (
    users::id,
    users::username,
    users::password,
);

type All = Select<users::table, AllColumns>;
type WithName<'a> = Eq<users::username, &'a str>;
type ByUsername<'a> = Filter<All, WithName<'a>>;

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
