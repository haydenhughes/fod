use super::All;
use crate::schema::meal_types;
use diesel::dsl::{Eq, Filter};
use diesel::prelude::*;
use fodmap_common::CreateMealType as ApiNewMealType;
use fodmap_common::MealType as ApiMealType;

type WithID<'a> = Eq<meal_types::id, &'a i32>;
type ByID<'a> = Filter<All<meal_types::table>, WithID<'a>>;

type WithName<'a> = Eq<meal_types::name, &'a str>;
type ByName<'a> = Filter<All<meal_types::table>, WithName<'a>>;

#[derive(Insertable, AsChangeset)]
#[table_name = "meal_types"]
pub struct NewMealType {
    pub name: String,
}

impl From<ApiNewMealType> for NewMealType {
    fn from(other: ApiNewMealType) -> Self {
        Self { name: other.name }
    }
}

#[derive(Identifiable, Queryable)]
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

    pub fn to_api(&self) -> ApiMealType {
        ApiMealType {
            id: self.id,
            name: self.name,
        }
    }
}
