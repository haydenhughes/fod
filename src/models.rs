use crate::schema::items;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;

type AllColumns = (
    items::id,
    items::name,
    items::qty,
    items::req_qty,
    items::price,
    items::notes,
);

pub type All = Select<items::table, AllColumns>;

pub type WithID<'a> = Eq<items::id, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

pub type WithName<'a> = Eq<items::name, &'a str>;
pub type ByName<'a> = Filter<All, WithName<'a>>;

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub qty: i32,
    pub req_qty: Option<i32>,
    pub price: Option<f32>,
    pub notes: Option<String>,
}

impl Item {
    pub fn with_id(id: &i32) -> WithID {
        items::id.eq(id)
    }

    pub fn with_name(name: &str) -> WithName {
        items::name.eq(name)
    }

    pub fn all() -> All {
        items::table.select(items::all_columns)
    }

    pub fn by_name(name: &str) -> ByName {
        Self::all().filter(Self::with_name(name))
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
