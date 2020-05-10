use crate::schema::foods;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;

type AllColumns = (foods::id, foods::name);

pub type All = Select<foods::table, AllColumns>;

pub type WithID<'a> = Eq<foods::id, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;


#[derive(Queryable, Serialize, Identifiable, PartialEq, Debug)]
pub struct Food {
    pub id: i32,
    pub name: String,
}

impl Food {
    pub fn with_id(id: &i32) -> WithID {
        foods::id.eq(id)
    }

    pub fn all() -> All {
        foods::table.select(foods::all_columns)
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
