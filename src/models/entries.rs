use crate::schema::entries;
use chrono::NaiveDateTime;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;
use crate::auth::User;
use super::paginated::Paginate;

type AllColumns = (
    entries::id,
    entries::user_id,
    entries::timestamp,
    entries::comments,
);

pub type All = Select<entries::table, AllColumns>;

pub type WithID<'a> = Eq<entries::id, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

pub type WithUserID<'a> = Eq<entries::user_id, &'a i32>;
pub type ByUserID<'a> = Filter<All, WithUserID<'a>>;

pub type WithTimeStamp<'a> = Eq<entries::timestamp, &'a NaiveDateTime>;
pub type ByTimeStamp<'a> = Filter<All, WithTimeStamp<'a>>;

#[derive(Queryable, Serialize, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "entries"]
#[belongs_to(User)]
pub struct Entry {
    pub id: i32,
    pub user_id: i32,
    pub timestamp: NaiveDateTime,
    pub comments: Option<String>,
}

impl Entry {
    pub fn with_id(id: &i32) -> WithID {
        entries::id.eq(id)
    }

    pub fn with_timestamp(timestamp: &NaiveDateTime) -> WithTimeStamp {
        entries::timestamp.eq(timestamp)
    }

    pub fn with_user_id(id: &i32) -> WithUserID {
        entries::user_id.eq(id)
    }

    pub fn all() -> All {
        entries::table.select(entries::all_columns)
    }

    pub fn by_timestamp(timestamp: &NaiveDateTime) -> ByTimeStamp {
        Self::all().filter(Self::with_timestamp(timestamp))
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }

    pub fn by_user_id(id: &i32) -> ByUserID {
        Self::all().filter(Self::with_user_id(id))
    }
}
