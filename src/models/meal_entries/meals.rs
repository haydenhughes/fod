use super::MealEntry;
use crate::schema::meals;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;

type AllColumns = (meals::food_id, meals::meal_entry_id);

pub type All = Select<meals::table, AllColumns>;

pub type WithMealEntryID<'a> = Eq<meals::meal_entry_id, &'a i32>;
pub type ByMealEntryID<'a> = Filter<All, WithMealEntryID<'a>>;

#[derive(Queryable, Serialize, Identifiable, Associations, PartialEq, Debug)]
#[belongs_to(MealEntry)]
#[primary_key(food_id, meal_entry_id)]
pub struct Meal {
    pub food_id: i32,
    pub meal_entry_id: i32,
}

impl Meal {
    pub fn with_meal_entry_id(id: &i32) -> WithMealEntryID {
        meals::meal_entry_id.eq(id)
    }

    pub fn all() -> All {
        meals::table.select(meals::all_columns)
    }

    pub fn by_meal_entry_id(id: &i32) -> ByMealEntryID {
        Self::all().filter(Self::with_meal_entry_id(id))
    }
}
