pub mod foods;
pub mod meals;
pub mod mealtypes;

use crate::schema::mealentries;
use chrono::naive::NaiveDateTime;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;

type AllColumns = (
    mealentries::mealentryid,
    mealentries::timestamp,
    mealentries::mealtype,
    mealentries::comments,
    mealentries::userid,
);

pub type All = Select<mealentries::table, AllColumns>;

pub type WithID<'a> = Eq<mealentries::mealentryid, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

pub type WithUserID<'a> = Eq<mealentries::userid, &'a i32>;
pub type ByUserID<'a> = Filter<All, WithUserID<'a>>;

pub type WithTimeStamp<'a> = Eq<mealentries::timestamp, &'a NaiveDateTime>;
pub type ByTimeStamp<'a> = Filter<All, WithTimeStamp<'a>>;

#[derive(Queryable, Serialize)]
pub struct MealEntry {
    pub mealentryid: i32,
    pub timestamp: NaiveDateTime,
    pub mealtype: i32,
    pub comments: Option<String>,
    pub userid: i32,
}

impl MealEntry {
    pub fn with_id(id: &i32) -> WithID {
        mealentries::mealentryid.eq(id)
    }

    pub fn with_timestamp(timestamp: &NaiveDateTime) -> WithTimeStamp {
        mealentries::timestamp.eq(timestamp)
    }

    pub fn with_user_id(id: &i32) -> WithUserID {
        mealentries::userid.eq(id)
    }

    pub fn all() -> All {
        mealentries::table.select(mealentries::all_columns)
    }

    pub fn by_timestamp(timestamp: &NaiveDateTime) -> ByTimeStamp {
        Self::all().filter(Self::with_timestamp(timestamp))
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }

    pub fn by_user_id(id: &i32) -> ByUserID {
        Self::all().filter(Self::with_user_id(id))
    }
}
