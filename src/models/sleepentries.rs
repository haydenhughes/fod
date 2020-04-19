use crate::schema::sleepentries;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;
use chrono::NaiveDateTime;

type AllColumns = (
    sleepentries::sleepentryid,
    sleepentries::endtime,
    sleepentries::comments,
);

pub type All = Select<sleepentries::table, AllColumns>;

pub type WithID<'a> = Eq<sleepentries::sleepentryid, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

#[derive(Queryable, Serialize)]
pub struct SleepEntry {
    pub id: i32,
    pub endtime: NaiveDateTime,
    pub comments: Option<String>,
}

impl SleepEntry {
    pub fn with_id(id: &i32) -> WithID {
        sleepentries::sleepentryid.eq(id)
    }

    pub fn all() -> All {
        sleepentries::table.select(sleepentries::all_columns)
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
