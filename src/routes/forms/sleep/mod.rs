mod context;

use crate::auth::User;
use crate::models::entries::Entry;
use crate::schema;
use crate::FodMapDatabase;
use context::NewSleepEntryForm;
use diesel::prelude::*;
use rocket::request::Form;
use rocket::response::Redirect;

#[post("/sleep_entry", data = "<sleep_form>")]
pub fn new_sleep_entry(
    user: User,
    conn: FodMapDatabase,
    sleep_form: Form<NewSleepEntryForm>,
) -> Result<Redirect, diesel::result::Error> {
    let entry: Entry = diesel::insert_into(schema::entries::table)
        .values(sleep_form.as_entry(user.id))
        .get_result(&conn.0)?;

    diesel::insert_into(schema::sleep_entries::table)
        .values(sleep_form.as_sleep_entry(entry.id))
        .execute(&conn.0)?;

    Ok(Redirect::to(uri!(crate::routes::index::index)))
}
