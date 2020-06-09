use super::{All, User};
use crate::schema::entries;
use chrono::NaiveDateTime;
use diesel::dsl::{Eq, Filter};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

type WithID<'a> = Eq<entries::id, &'a i32>;
type ByID<'a> = Filter<All<entries::table>, WithID<'a>>;

type WithUser<'a> = Eq<entries::user_id, &'a i32>;
type ByUser<'a> = Filter<All<entries::table>, WithUser<'a>>;

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "entries"]
pub struct NewEntry {
    pub user_id: i32,
    pub meal_type_id: i32,
    pub timestamp: NaiveDateTime,
    pub comments: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, Serialize)]
#[belongs_to(User)]
#[table_name = "entries"]
pub struct Entry {
    pub id: i32,
    pub user_id: i32,
    pub meal_type_id: i32,
    pub timestamp: NaiveDateTime,
    pub comments: Option<String>,
}

impl Entry {
    pub fn all() -> All<entries::table> {
        entries::table.select(entries::all_columns)
    }

    pub fn with_id(id: &i32) -> WithID {
        entries::id.eq(id)
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }

    pub fn with_user(user: &User) -> WithUser {
        entries::user_id.eq(&user.id)
    }

    pub fn by_user(user: &User) -> ByUser {
        Self::all().filter(Self::with_user(user))
    }
}
