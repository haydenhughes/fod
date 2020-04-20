pub mod exercisetypes;

use super::entries::Entry;
use crate::schema::exercise_entries;
use chrono::NaiveDateTime;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
pub use exercisetypes::ExerciseType;
use serde::Serialize;

type AllColumns = (
    exercise_entries::id,
    exercise_entries::entry_id,
    exercise_entries::exercise_type_id,
    exercise_entries::duration,
);

pub type All = Select<exercise_entries::table, AllColumns>;

pub type WithID<'a> = Eq<exercise_entries::id, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

#[derive(Queryable, Serialize, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "exercise_entries"]
#[belongs_to(Entry)]
pub struct ExerciseEntry {
    pub id: i32,
    pub entry_id: i32,
    pub exercise_type_id: i32,
    pub timestamp: NaiveDateTime,
}

impl ExerciseEntry {
    pub fn with_id(id: &i32) -> WithID {
        exercise_entries::id.eq(id)
    }

    pub fn all() -> All {
        exercise_entries::table.select(exercise_entries::all_columns)
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
