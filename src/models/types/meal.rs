use crate::schema::meal_types;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;

type AllColumns = (meal_types::id, meal_types::name);

pub type All = Select<meal_types::table, AllColumns>;

pub type WithID<'a> = Eq<meal_types::id, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

#[derive(Queryable, Serialize)]
pub struct MealType {
    pub id: i32,
    pub name: String,
}

impl MealType {
    pub fn with_id(id: &i32) -> WithID {
        meal_types::id.eq(id)
    }

    pub fn all() -> All {
        meal_types::table.select(meal_types::all_columns)
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
