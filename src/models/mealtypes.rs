use crate::schema::mealtypes;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;

type AllColumns = (
    mealtypes::mealtypeid,
    mealtypes::name,
);

pub type All = Select<mealtypes::table, AllColumns>;

pub type WithID<'a> = Eq<mealtypes::mealtypeid, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

pub type WithName<'a> = Eq<mealtypes::name, &'a str>;
pub type ByName<'a> = Filter<All, WithName<'a>>;

#[derive(Queryable, Serialize)]
pub struct MealType {
    pub id: i32,
    pub name: String,
}

impl MealType {
    pub fn with_id(id: &i32) -> WithID {
        mealtypes::mealtypeid.eq(id)
    }

    pub fn with_name(name: &str) -> WithName {
        mealtypes::name.eq(name)
    }

    pub fn all() -> All {
        mealtypes::table.select(mealtypes::all_columns)
    }

    pub fn by_name(name: &str) -> ByName {
        Self::all().filter(Self::with_name(name))
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
