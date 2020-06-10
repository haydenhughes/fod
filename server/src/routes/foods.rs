use crate::models::{self, User};
use crate::schema::foods;
use crate::FodmapDbConn;
use diesel::prelude::*;
use fodmap_common::{CreateFood, Food};
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/<id>")]
pub fn get_food(
    _user: User,
    conn: FodmapDbConn,
    id: Option<i32>,
) -> Result<Json<Vec<Food>>, status::NotFound<&'static str>> {
    match id {
        Some(id) => models::Food::by_id(&id)
            .get_results::<models::Food>(&*conn)
            .map_err(|e| {
                warn!("Unable to query meal type {}", e);
                status::NotFound("The queried food doesn't exist")
            })
            .map(|r| r.iter().map(|e| e.to_api()).collect())
            .map(Json),

        None => Ok(Json(
            models::Food::all()
                .get_results::<models::Food>(&*conn)
                .expect("Unable to query food")
                .iter()
                .map(|e| e.to_api())
                .collect::<Vec<Food>>(),
        )),
    }
}

#[post("/", data = "<content>")]
pub fn create_food(
    _user: User,
    conn: FodmapDbConn,
    content: Json<CreateFood>,
) -> Result<status::Created<Json<Food>>, status::Conflict<&'static str>> {
    diesel::insert_into(foods::table)
        .values(models::NewFood::from(content.into_inner()))
        .get_result::<models::Food>(&*conn)
        .map(|r| r.to_api())
        .map(Json)
        .map(|r| status::Created(uri!(get_food: r.id).to_string(), Some(r)))
        .map_err(|e| {
            warn!("Unable to insert food: {}", e);
            status::Conflict(Some("Food already exists"))
        })
}

#[put("/<id>", data = "<content>")]
pub fn update_food(
    _user: User,
    conn: FodmapDbConn,
    id: i32,
    content: Json<CreateFood>,
) -> Result<Json<Food>, status::NotFound<&'static str>> {
    models::Food::by_id(&id)
        .get_result::<models::Food>(&*conn)
        .map_err(|e| {
            warn!("Unable to query food: {}", e);
            status::NotFound("The food queried does not exist")
        })
        .map(|meal_type| {
            diesel::update(&meal_type)
                .set(models::NewFood::from(content.into_inner()))
                .get_result::<models::Food>(&*conn)
                .map(|r| r.to_api())
                .map(Json)
                .expect("Unable to update food")
        })
}

#[delete("/<id>")]
pub fn delete_meal_type(
    _user: User,
    conn: FodmapDbConn,
    id: i32,
) -> Result<status::NoContent, status::Conflict<&'static str>> {
    match models::MealType::by_id(&id).get_result::<models::MealType>(&*conn) {
        Ok(meal_type) => diesel::delete(&meal_type)
            .execute(&*conn)
            .map(|_| status::NoContent)
            .map_err(|e| {
                warn!("Unable to delete food: {}", e);
                status::Conflict(Some(
                    "Unable to delete food, possibly required by an entry",
                ))
            }),
        Err(e) => {
            warn!("Unable to query food: {}", e);
            Ok(status::NoContent)
        }
    }
}
