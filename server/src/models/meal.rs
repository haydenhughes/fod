use super::All;
use super::Entry;
use crate::schema::meals;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Associations)]
#[primary_key(food_id, entry_id)]
#[belongs_to(Entry)]
pub struct Meal {
    pub food_id: i32,
    pub entry_id: i32,
}

impl Meal {
    pub fn all() -> All<meals::table> {
        meals::table.select(meals::all_columns)
    }
}
