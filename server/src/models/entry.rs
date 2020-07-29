use super::{All, Food, Meal, MealType, User};
use crate::schema::entries;
use chrono::NaiveDateTime;
use diesel::dsl::{Eq, Filter};
use diesel::prelude::*;
use fodmap_common::{
    CreateEntry as ApiNewEntry, Entry as ApiEntry, Food as ApiFood, MealType as ApiMealType,
};

type WithID<'a> = Eq<entries::id, &'a i32>;
type ByID<'a> = Filter<All<entries::table>, WithID<'a>>;

type WithUser<'a> = Eq<entries::user_id, &'a i32>;
type ByUser<'a> = Filter<All<entries::table>, WithUser<'a>>;

#[derive(Insertable, AsChangeset)]
#[table_name = "entries"]
pub struct NewEntry {
    user_id: i32,
    meal_type_id: i32,
    hunger_before: i32,
    hunger_after: i32,
    timestamp: NaiveDateTime,
    comments: Option<String>,
}

impl NewEntry {
    pub fn new(user: User, entry: ApiNewEntry) -> Self {
        Self {
            user_id: user.id,
            meal_type_id: entry.meal_type.id,
            hunger_before: entry.hunger_before,
            hunger_after: entry.hunger_after,
            timestamp: entry.timestamp,
            comments: entry.comments,
        }
    }
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User)]
#[table_name = "entries"]
pub struct Entry {
    pub id: i32,
    pub user_id: i32,
    pub meal_type_id: i32,
    pub hunger_before: i32,
    pub hunger_after: i32,
    pub timestamp: NaiveDateTime,
    pub comments: Option<String>,
}

impl Entry {
    pub fn all() -> All<entries::table> {
        entries::table.select(entries::all_columns)
    }

    pub fn with_id(id: &i32) -> WithID {
        entries::id.eq(id)
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }

    pub fn with_user(user: &User) -> WithUser {
        entries::user_id.eq(&user.id)
    }

    pub fn by_user(user: &User) -> ByUser {
        Self::all().filter(Self::with_user(user))
    }

    pub fn to_api(&self, conn: &PgConnection) -> ApiEntry {
        let meal_type = MealType::by_id(&self.meal_type_id)
            .get_result::<MealType>(conn)
            .expect("Unable to query meal type")
            .to_api();
        let foods = Meal::belonging_to(self)
            .get_results::<Meal>(conn)
            .expect("Unable to query meals")
            .iter()
            .map(|meal| {
                Food::by_id(&meal.food_id)
                    .get_result::<Food>(conn)
                    .expect("Unable to query food")
                    .to_api()
            })
            .collect();

        ApiEntry {
            id: self.id,
            meal_type,
            hunger_before: self.hunger_before,
            hunger_after: self.hunger_after,
            timestamp: self.timestamp,
            foods,
            comments: self.comments,
        }
    }
}
