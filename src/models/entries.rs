use crate::schema::entries;
use chrono::naive::NaiveDateTime;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;

type AllColumns = (
    entries::entryid,
    entries::userid,
    entries::mealentryid,
    entries::exerciseentryid,
    entries::sleepentryid,
    entries::timestamp,
);

pub type All = Select<entries::table, AllColumns>;

pub type WithID<'a> = Eq<entries::entryid, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

pub type WithUserID<'a> = Eq<entries::userid, &'a i32>;
pub type ByUserID<'a> = Filter<All, WithUserID<'a>>;

pub type WithTimeStamp<'a> = Eq<entries::timestamp, &'a NaiveDateTime>;
pub type ByTimeStamp<'a> = Filter<All, WithTimeStamp<'a>>;

#[derive(Queryable, Serialize)]
pub struct Entry {
    pub id: i32,
    pub userid: i32,
    pub mealentryid: i32,
    pub exerciseentryid: i32,
    pub sleepentryid: i32,
    pub timestamp: NaiveDateTime,
}

impl Entry {
    pub fn with_id(id: &i32) -> WithID {
        entries::entryid.eq(id)
    }

    pub fn with_timestamp(timestamp: &NaiveDateTime) -> WithTimeStamp {
        entries::timestamp.eq(timestamp)
    }

    pub fn with_user_id(id: &i32) -> WithUserID {
        entries::userid.eq(id)
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
