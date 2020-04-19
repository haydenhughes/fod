use crate::schema::sleepentries;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;
use chrono::NaiveDateTime;

type AllColumns = (
    sleepentries::sleepentryid,
    sleepentries::userid,
    sleepentries::starttime,
    sleepentries::endtime,
    sleepentries::comments,
);

pub type All = Select<sleepentries::table, AllColumns>;

pub type WithID<'a> = Eq<sleepentries::sleepentryid, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

pub type WithUserID<'a> = Eq<sleepentries::userid, &'a i32>;
pub type ByUserID<'a> = Filter<All, WithUserID<'a>>;

#[derive(Queryable, Serialize)]
pub struct SleepEntry {
    pub id: i32,
    pub userid: i32,
    pub starttime: NaiveDateTime,
    pub endtime: NaiveDateTime,
    pub comments: Option<String>,
}

impl SleepEntry {
    pub fn with_id(id: &i32) -> WithID {
        sleepentries::sleepentryid.eq(id)
    }

    pub fn with_user_id(id: &i32) -> WithUserID {
        sleepentries::userid.eq(id)
    }

    pub fn all() -> All {
        sleepentries::table.select(sleepentries::all_columns)
    }

    pub fn by_user_id(id: &i32) -> ByUserID {
        Self::all().filter(Self::with_user_id(id))
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
