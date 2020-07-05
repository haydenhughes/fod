use crate::models::{self, User};
use crate::schema::{entries, meal_types};
use crate::FodmapDbConn;
use diesel::prelude::*;
use fodmap_common::{CreateEntry, Entry, Food};
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/<id>")]
pub fn get_entry(
    _user: User,
    conn: FodmapDbConn,
    id: Option<i32>,
) -> Result<Json<Vec<Entry>>, status::NotFound<&'static str>> {
    match id {
        Some(id) => models::Entry::by_id(&id)
            .get_results::<models::Entry>(&*conn)
            .map(|r| r.iter().map(|e| e.to_api(&*conn)).collect())
            .map(Json)
            .map_err(|e| {
                warn!("Unable to query entry: {}", e);
                status::NotFound("Entry not found")
            }),
        None => Ok(Json(
            models::Entry::all()
                .get_results::<models::Entry>(&*conn)
                .expect("Unable to query entries")
                .iter()
                .map(|e| e.to_api(&*conn))
                .collect(),
        )),
    }
}

#[post("/", data = "<content>")]
pub fn create_entry(
    user: User,
    conn: FodmapDbConn,
    content: Json<CreateEntry>,
) -> Result<status::Created<Json<Entry>>, status::BadRequest<&'static str>> {
    // TODO: Create meals

    diesel::insert_into(entries::table)
        .values(models::NewEntry::from((user, content.into_inner())))
        .get_result::<models::Entry>(&*conn)
        .map(|e| e.to_api(&*conn))
        .map(Json)
        .map(|e| status::Created(uri!(get_entry: e.id).to_string(), Some(e)))
        .map_err(|e| {
            warn!("Unable to insert entry: {}", e);
            status::BadRequest(Some("Unable to create entry, is the "))
        })
}

#[put("/<id>", data = "<entry>")]
pub fn update_entry(
    user: User,
    conn: FodmapDbConn,
    id: i32,
    entry: Json<Entry>,
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
                .set(to_new_entry(&user, &*conn, &entry))
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
