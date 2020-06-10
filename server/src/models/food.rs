use super::All;
use crate::schema::foods;
use diesel::dsl::{Eq, Filter};
use diesel::prelude::*;
use fodmap_common::CreateFood as ApiNewFood;
use fodmap_common::Food as ApiFood;

type WithID<'a> = Eq<foods::id, &'a i32>;
type ByID<'a> = Filter<All<foods::table>, WithID<'a>>;

#[derive(Insertable, AsChangeset)]
#[table_name = "foods"]
pub struct NewFood {
    name: String,
}

impl From<ApiNewFood> for NewFood {
    fn from(other: ApiNewFood) -> Self {
        Self { name: other.name }
    }
}

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

    pub fn to_api(&self) -> ApiFood {
        ApiFood {
            id: self.id,
            name: self.name,
        }
    }
}
