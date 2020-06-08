use super::All;
use crate::schema::meal_types;
use diesel::dsl::{Eq, Filter};
use diesel::prelude::*;

type WithID<'a> = Eq<meal_types::id, &'a i32>;
type ByID<'a> = Filter<All<meal_types::table>, WithID<'a>>;

#[derive(Queryable)]
pub struct MealType {
    pub id: i32,
    pub name: String,
}

impl MealType {
    pub fn all() -> All<meal_types::table> {
        meal_types::table.select(meal_types::all_columns)
    }

    pub fn with_id(id: &i32) -> WithID {
        meal_types::id.eq(id)
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
