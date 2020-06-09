use crate::models::{User, Entry};
use crate::FodmapDbConn;
use diesel::prelude::*;
use rocket_contrib::json::Json;


#[get("/entries")]
fn list_entries(user: User, conn: FodmapDbConn) -> Result<Json<User>, diesel::result::Error> {
    Ok(Json(Entry::belonging_to(user).all().get_results::<Entry>(&*conn)?))
}
