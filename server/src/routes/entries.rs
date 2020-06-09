use crate::models::{Entry, MealType, NewEntry, NewMealType, User};
use crate::schema::{entries, meal_types};
use crate::FodmapDbConn;
use diesel::prelude::*;
use fodmap_common::CreateEntry;
use rocket::response::status;
use rocket_contrib::json::Json;

fn create_new_entry(user: &User, conn: &PgConnection, entry: &Json<CreateEntry>) -> NewEntry {
    NewEntry {
        user_id: user.id,
        meal_type_id: {
            if let Ok(meal_type) = MealType::by_name(&entry.meal_type).get_result::<MealType>(conn)
            {
                meal_type.id
            } else {
                diesel::insert_into(meal_types::table)
                    .values(NewMealType {
                        name: entry.meal_type.to_owned(),
                    })
                    .get_result::<MealType>(conn)
                    .expect("Unable to create meal type")
                    .id
            }
        },
        timestamp: entry.timestamp,
        comments: entry.comments.to_owned(),
    }
}

#[get("/<page>?<page_size>")]
pub fn list_entries(
    user: User,
    conn: FodmapDbConn,
    page: i64,
    page_size: Option<i64>,
) -> Json<Vec<Entry>> {
    Entry::belonging_to(&user)
        .order(entries::timestamp.desc())
        .offset(page * page_size.unwrap_or(20))
        .limit(page_size.unwrap_or(20))
        .get_results::<Entry>(&*conn)
        .map(|r| Json(r))
        .expect("Unable to query entries")
}

#[get("/<id>")]
pub fn get_entry(
    _user: User,
    conn: FodmapDbConn,
    id: i32,
) -> Result<Json<Entry>, status::NotFound<&'static str>> {
    Entry::by_id(&id)
        .get_result(&*conn)
        .map(|r| Json(r))
        .map_err(|e| {
            warn!("Unable to query entry: {}", e);
            status::NotFound("Entry not found")
        })
}

#[post("/", data = "<entry>")]
pub fn create_entry(
    user: User,
    conn: FodmapDbConn,
    entry: Json<CreateEntry>,
) -> status::Created<Json<Entry>> {
    diesel::insert_into(entries::table)
        .values(create_new_entry(&user, &*conn, &entry))
        .get_result::<Entry>(&*conn)
        .map(|r| Json(r))
        .map(|r| status::Created(uri!(get_entry: r.id).to_string(), Some(r)))
        .expect("Unable to create entry")
}

#[put("/<id>", data = "<entry>")]
pub fn update_entry(
    user: User,
    conn: FodmapDbConn,
    id: i32,
    entry: Json<CreateEntry>,
) -> Result<Json<Entry>, status::NotFound<&'static str>> {
    Entry::belonging_to(&user)
        .filter(Entry::with_id(&id))
        .get_result::<Entry>(&*conn)
        .map_err(|e| {
            warn!("Unable to query entry: {}", e);
            status::NotFound("Entry not found")
        })
        .and_then(|e| {
            Ok(diesel::update(&e)
                .set(create_new_entry(&user, &*conn, &entry))
                .get_result::<Entry>(&*conn)
                .map(|r| Json(r))
                .expect("Unable to update entry"))
        })
}

#[delete("/<id>")]
pub fn delete_entry(
    user: User,
    conn: FodmapDbConn,
    id: i32,
) -> Result<status::NoContent, status::Forbidden<&'static str>> {
    Entry::belonging_to(&user)
        .filter(Entry::with_id(&id))
        .get_result::<Entry>(&*conn)
        .map_err(|e| {
            warn!("Unable to delete entry: {}", e);
            status::Forbidden(Some("Forbidden from deleting entry"))
        })
        .and_then(|entry| {
            Ok(diesel::delete(&entry)
                .execute(&*conn)
                .map(|_| status::NoContent)
                .expect("Unable to delete entry"))
        })
}
