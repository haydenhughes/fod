pub mod foods;
pub mod meals;
pub mod mealtypes;

use crate::schema::mealentries;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;

type AllColumns = (
    mealentries::mealentryid,
    mealentries::mealtype,
    mealentries::comments,
);

pub type All = Select<mealentries::table, AllColumns>;

pub type WithID<'a> = Eq<mealentries::mealentryid, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

#[derive(Queryable, Serialize)]
pub struct MealEntry {
    pub id: i32,
    pub mealtype: i32,
    pub comments: Option<String>,
}

impl MealEntry {
    pub fn with_id(id: &i32) -> WithID {
        mealentries::mealentryid.eq(id)
    }

    pub fn all() -> All {
        mealentries::table.select(mealentries::all_columns)
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
