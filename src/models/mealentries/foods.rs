use crate::schema::foods;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;

type AllColumns = (
    foods::foodid,
    foods::name,
);

pub type All = Select<foods::table, AllColumns>;

pub type WithID<'a> = Eq<foods::foodid, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

pub type WithName<'a> = Eq<foods::name, &'a str>;
pub type ByName<'a> = Filter<All, WithName<'a>>;

#[derive(Queryable, Serialize)]
pub struct Food {
    pub id: i32,
    pub name: String,
}

impl Food {
    pub fn with_id(id: &i32) -> WithID {
        foods::foodid.eq(id)
    }

    pub fn with_name(name: &str) -> WithName {
        foods::name.eq(name)
    }

    pub fn all() -> All {
        foods::table.select(foods::all_columns)
    }

    pub fn by_name(name: &str) -> ByName {
        Self::all().filter(Self::with_name(name))
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
