use super::All;
use crate::schema::meal_types;
use diesel::dsl::{Eq, Filter};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

type WithID<'a> = Eq<meal_types::id, &'a i32>;
type ByID<'a> = Filter<All<meal_types::table>, WithID<'a>>;

type WithName<'a> = Eq<meal_types::name, &'a str>;
type ByName<'a> = Filter<All<meal_types::table>, WithName<'a>>;

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "meal_types"]
pub struct NewMealType {
    name: String,
}

impl NewMealType {
    pub fn new<S: Into<String>>(name: S) -> Self {
        NewMealType { name: name.into() }
    }
}

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
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

    pub fn with_name(name: &str) -> WithName {
        meal_types::name.eq(name)
    }

    pub fn by_name(name: &str) -> ByName {
        Self::all().filter(Self::with_name(name))
    }
}
