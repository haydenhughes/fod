use crate::models::{self, User};
use crate::schema::meal_types;
use crate::FodmapDbConn;
use diesel::prelude::*;
use fodmap_common::{CreateMealType, MealType};
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/<id>")]
pub fn get_meal_type(
    _user: User,
    conn: FodmapDbConn,
    id: Option<i32>,
) -> Result<Json<Vec<MealType>>, status::NotFound<&'static str>> {
    match id {
        Some(id) => models::MealType::by_id(&id)
            .get_results::<models::MealType>(&*conn)
            .map_err(|e| {
                warn!("Unable to query meal type {}", e);
                status::NotFound("The queried meal type does not exist")
            })
            .map(|r| r.iter().map(|e| e.to_api()).collect())
            .map(Json),

        None => Ok(Json(
            models::MealType::all()
                .get_results::<models::MealType>(&*conn)
                .expect("Unable to query meal types")
                .iter()
                .map(|e| e.to_api())
                .collect::<Vec<MealType>>(),
        )),
    }
}

#[post("/", data = "<content>")]
pub fn create_meal_type(
    _user: User,
    conn: FodmapDbConn,
    content: Json<CreateMealType>,
) -> Result<status::Created<Json<MealType>>, status::Conflict<&'static str>> {
    diesel::insert_into(meal_types::table)
        .values(models::NewMealType::from(content.into_inner()))
        .get_result::<models::MealType>(&*conn)
        .map(|r| r.to_api())
        .map(Json)
        .map(|r| status::Created(uri!(get_meal_type: r.id).to_string(), Some(r)))
        .map_err(|e| {
            warn!("Unable to insert meal type: {}", e);
            status::Conflict(Some("Meal type already exists"))
        })
}

#[put("/<id>", data = "<content>")]
pub fn update_meal_type(
    _user: User,
    conn: FodmapDbConn,
    id: i32,
    content: Json<CreateMealType>,
) -> Result<Json<MealType>, status::NotFound<&'static str>> {
    models::MealType::by_id(&id)
        .get_result::<models::MealType>(&*conn)
        .map_err(|e| {
            warn!("Unable to query meal type: {}", e);
            status::NotFound("Meal type not found")
        })
        .map(|meal_type| {
            diesel::update(&meal_type)
                .set(models::NewMealType::from(content.into_inner()))
                .get_result::<models::MealType>(&*conn)
                .map(|r| r.to_api())
                .map(Json)
                .expect("Unable to update meal type")
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
                warn!("Unable to delete meal type: {}", e);
                status::Conflict(Some(
                    "Unable to delete meal type, possibly required by an entry",
                ))
            }),
        Err(e) => {
            warn!("Unable to query meal type: {}", e);
            Ok(status::NoContent)
        }
    }
}
