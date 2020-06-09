use crate::models::{MealType, NewMealType, User};
use crate::schema::meal_types;
use crate::FodmapDbConn;
use diesel::prelude::*;
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/")]
pub fn list_meal_types(_user: User, conn: FodmapDbConn) -> Json<Vec<MealType>> {
    MealType::all()
        .get_results(&*conn)
        .map(|r| Json(r))
        .expect("Unable to query meal types")
}

#[get("/<id>")]
pub fn get_meal_type(
    _user: User,
    conn: FodmapDbConn,
    id: i32,
) -> Result<Json<MealType>, status::NotFound<&'static str>> {
    MealType::by_id(&id)
        .get_result(&*conn)
        .map_err(|e| {
            warn!("Unable to query meal type {}", e);
            status::NotFound("Meal type not found")
        })
        .map(|r| Json(r))
}

#[post("/", data = "<meal_type>")]
pub fn create_meal_type(
    _user: User,
    conn: FodmapDbConn,
    meal_type: Json<NewMealType>,
) -> Result<status::Created<Json<MealType>>, status::Conflict<&'static str>> {
    diesel::insert_into(meal_types::table)
        .values(meal_type.into_inner())
        .get_result::<MealType>(&*conn)
        .map(|r| status::Created(uri!(get_meal_type: r.id).to_string(), Some(Json(r))))
        .map_err(|e| {
            warn!("Unable to insert meal type: {}", e);
            status::Conflict(Some("Meal type already exists"))
        })
}

#[put("/<id>", data = "<update>")]
pub fn update_meal_type(
    _user: User,
    conn: FodmapDbConn,
    id: i32,
    update: Json<NewMealType>,
) -> Result<Json<MealType>, status::NotFound<&'static str>> {
    MealType::by_id(&id)
        .get_result::<MealType>(&*conn)
        .map_err(|e| {
            warn!("Unable to query meal type: {}", e);
            status::NotFound("Meal type not found")
        })
        .and_then(|meal_type| {
            Ok(diesel::update(&meal_type)
                .set(update.into_inner())
                .get_result(&*conn)
                .map(|r| Json(r))
                .expect("Unable to update meal type"))
        })
}

#[delete("/<id>")]
pub fn delete_meal_type(
    _user: User,
    conn: FodmapDbConn,
    id: i32,
) -> Result<status::NoContent, status::Conflict<&'static str>> {
    match MealType::by_id(&id).get_result::<MealType>(&*conn) {
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
            warn!("Unable to query meal_type: {}", e);
            Ok(status::NoContent)
        }
    }
}
