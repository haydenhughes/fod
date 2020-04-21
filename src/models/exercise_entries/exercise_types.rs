use crate::schema::exercise_types;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;

type AllColumns = (
    exercise_types::id,
    exercise_types::name,
);

pub type All = Select<exercise_types::table, AllColumns>;

pub type WithID<'a> = Eq<exercise_types::id, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

#[derive(Queryable, Serialize)]
pub struct ExerciseType {
    pub id: i32,
    pub name: String,
}

impl ExerciseType {
    pub fn with_id(id: &i32) -> WithID {
        exercise_types::id.eq(id)
    }

    pub fn all() -> All {
        exercise_types::table.select(exercisetypes::all_columns)
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
