use super::Entry;
use super::{ExerciseEntry, MealEntry, SleepEntry};
use crate::auth::User;
use crate::schema::meta_entries;
use chrono::NaiveDateTime;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;

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
pub struct MetaEntry {
    pub id: i32,
    pub user_id: i32,
    pub timestamp: NaiveDateTime,
    pub comments: Option<String>,
}

impl MetaEntry {
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

    pub fn to_entry(&self, conn: &PgConnection) -> Result<Entry, diesel::result::Error> {
        let meal_entry = MealEntry::belonging_to(self)
            .first::<MealEntry>(conn)
            .map(|e| EntryType::Meal(e));
        let sleep_entry = SleepEntry::belonging_to(self)
            .first::<SleepEntry>(conn)
            .map(|e| EntryType::Sleep(e));
        let exercise_entry = ExerciseEntry::belonging_to(self)
            .first::<ExerciseEntry>(conn)
            .map(|e| EntryType::Exercise(e));

        meal_entry.or(sleep_entry).or(exercise_entry)
    }
}
