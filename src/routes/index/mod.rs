mod context;

use crate::auth::User;
use crate::models::entries::MetaEntry;
use crate::models::misc::{Food, Meal};
use crate::models::types::{ExerciseType, MealType};
use crate::FodMapDatabase;
use context::{EntryContext, IndexContext};
use diesel::prelude::*;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(user: User, conn: FodMapDatabase) -> Result<Template, diesel::result::Error> {
    let entries = MetaEntry::by_user_id(&user.id).get_results::<MetaEntry>(&conn.0)?;
    let context = IndexContext::new(
        entries
            .iter()
            .map(|m| m.to_entry(&conn.0).and_then(|e| Ok(EntryContext::from(e))))
            .collect::<Result<Vec<EntryContext>, diesel::result::Error>>()?,
        Food::all().get_results::<Food>(&conn.0)?,
        MealType::all().get_results::<MealType>(&conn.0)?,
        ExerciseType::all().get_results::<ExerciseType>(&conn.0)?,
    );
    Ok(Template::render("index", context))
}
