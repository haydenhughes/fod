use crate::schema::meals;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;

type AllColumns = (
    meals::foodid,
    meals::entryid,
    meals::qty,
);

pub type All = Select<meals::table, AllColumns>;

pub type WithEntryID<'a> = Eq<meals::entryid, &'a i32>;
pub type ByEntryID<'a> = Filter<All, WithEntryID<'a>>;

#[derive(Queryable, Serialize)]
pub struct Meal {
    pub foodid: i32,
    pub enrtyid: i32,
    pub qty: i32,
}

impl Meal {
    pub fn with_entry_id(id: &i32) -> WithEntryID {
        meals::entryid.eq(id)
    }

    pub fn all() -> All {
        meals::table.select(meals::all_columns)
    }

    pub fn by_entry_id(id: &i32) -> ByEntryID {
        Self::all().filter(Self::with_entry_id(id))
    }
}
