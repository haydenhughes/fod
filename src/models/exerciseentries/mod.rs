use crate::schema::exerciseentries;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;

type AllColumns = (
    exerciseentries::exerciseentryid,
    exerciseentries::userid,
    exerciseentries::exercisetype,
    exerciseentries::starttime,
    exerciseentries::endtime,
    exerciseentries::comments,
);

pub type All = Select<exerciseentries::table, AllColumns>;

pub type WithID<'a> = Eq<exerciseentries::exerciseentryid, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

pub type WithUserID<'a> = Eq<exerciseentries::userid, &'a i32>;
pub type ByUserID<'a> = Filter<All, WithUserID<'a>>;

#[derive(Queryable, Serialize)]
pub struct ExerciseEntry {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl ExerciseEntry {
    pub fn with_id(id: &i32) -> WithID {
        exerciseentries::exerciseentryid.eq(id)
    }

    pub fn with_user_id(id: &i32) -> WithUserID {
        exerciseentries::userid.eq(id)
    }

    pub fn all() -> All {
        exerciseentries::table.select(exerciseentries::all_columns)
    }

    pub fn by_user_id(id: &i32) -> ByUserID {
        Self::all().filter(Self::with_user_id(id))
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
