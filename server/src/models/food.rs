use super::All;
use crate::schema::foods;
use diesel::dsl::{Eq, Filter};
use diesel::prelude::*;

type WithID<'a> = Eq<foods::id, &'a i32>;
type ByID<'a> = Filter<All<foods::table>, WithID<'a>>;

#[derive(Identifiable, Queryable)]
pub struct Food {
    pub id: i32,
    pub name: String,
}

impl Food {
    pub fn all() -> All<foods::table> {
        foods::table.select(foods::all_columns)
    }

    pub fn with_id(id: &i32) -> WithID {
        foods::id.eq(id)
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
