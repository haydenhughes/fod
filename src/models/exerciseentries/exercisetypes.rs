use crate::schema::exercisetypes;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;

type AllColumns = (
    exercisetypes::exercisetypeid,
    exercisetypes::name,
);

pub type All = Select<exercisetypes::table, AllColumns>;

pub type WithID<'a> = Eq<exercisetypes::exercisetypeid, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

pub type WithName<'a> = Eq<exercisetypes::name, &'a str>;
pub type ByName<'a> = Filter<All, WithName<'a>>;

#[derive(Queryable, Serialize)]
pub struct ExerciseType {
    pub id: i32,
    pub name: String,
}

impl ExerciseType {
    pub fn with_id(id: &i32) -> WithID {
        exercisetypes::exercisetypeid.eq(id)
    }

    pub fn with_name(name: &str) -> WithName {
        exercisetypes::name.eq(name)
    }

    pub fn all() -> All {
        exercisetypes::table.select(exercisetypes::all_columns)
    }

    pub fn by_name(name: &str) -> ByName {
        Self::all().filter(Self::with_name(name))
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
