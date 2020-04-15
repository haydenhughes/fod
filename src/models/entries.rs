use crate::schema::entries;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;
use chrono::naive::NaiveDateTime;

type AllColumns = (
    entries::entryid,
    entries::timestamp,
    entries::mealtype,
    entries::comments,
);

pub type All = Select<entries::table, AllColumns>;

pub type WithID<'a> = Eq<entries::entryid, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

pub type WithTimeStamp<'a> = Eq<entries::timestamp, &'a NaiveDateTime>;
pub type ByTimeStamp<'a> = Filter<All, WithTimeStamp<'a>>;

#[derive(Queryable, Serialize)]
pub struct Entry {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub mealtype: i32,
    pub comments: String,
}

impl Entry {
    pub fn with_id(id: &i32) -> WithID {
        entries::entryid.eq(id)
    }

    pub fn with_timestamp(timestamp: &NaiveDateTime) -> WithTimeStamp {
        entries::timestamp.eq(timestamp)
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
}
