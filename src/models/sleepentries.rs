use crate::schema::sleep_entries;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;
use chrono::NaiveDateTime;
use super::Entry;

type AllColumns = (
    sleep_entries::id,
    sleep_entries::entry_id,
    sleep_entries::duration,
);

pub type All = Select<sleep_entries::table, AllColumns>;

pub type WithID<'a> = Eq<sleep_entries::id, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

#[derive(Queryable, Serialize, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "sleep_entries"]
#[belongs_to(Entry)]
pub struct SleepEntry {
    pub id: i32,
    pub entry_id: i32,
    pub duration: NaiveDateTime,
}

impl SleepEntry {
    pub fn with_id(id: &i32) -> WithID {
        sleep_entries::id.eq(id)
    }

    pub fn all() -> All {
        sleep_entries::table.select(sleep_entries::all_columns)
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
