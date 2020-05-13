mod context;

use crate::auth::User;
use crate::models::types::{ExerciseType, MealType};
use crate::models::misc::{Food, Meal};
use crate::models::entries::{Entry, ExerciseEntry, MealEntry, SleepEntry};
use crate::FodMapDatabase;
use context::{EntryContext, IndexContext};
use diesel::prelude::*;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(user: User, conn: FodMapDatabase) -> Result<Template, diesel::result::Error> {
    let entries_query = Entry::by_user_id(&user.id).get_results::<Entry>(&conn.0)?;
    let entries = entries_query
        .iter()
        .map(|e| {
            let meal_entry = MealEntry::belonging_to(e).first::<MealEntry>(&conn.0);
            let sleep_entry = SleepEntry::belonging_to(e).first::<SleepEntry>(&conn.0);
            let exercise_entry = ExerciseEntry::belonging_to(e).first::<ExerciseEntry>(&conn.0);

            if let Ok(meal_entry) = meal_entry {
                EntryContext::Meal {
                    entry: e,
                    meal_type: MealType::by_id(&meal_entry.meal_type_id)
                        .first(&conn.0)
                        .expect("Unable to get meal type"),
                    foods: Meal::by_meal_entry_id(&meal_entry.id)
                        .get_results::<Meal>(&conn.0)
                        .expect("Unable to get meals")
                        .iter()
                        .map(|c| {
                            Food::by_id(&c.food_id)
                                .first(&conn.0)
                                .expect("Unable to get food")
                        })
                        .collect::<Vec<Food>>(),
                }
            } else if let Ok(sleep_entry) = sleep_entry {
                EntryContext::Sleep {
                    entry: e,
                    sleep: sleep_entry,
                }
            } else if let Ok(exercise_entry) = exercise_entry {
                EntryContext::Exercise {
                    entry: e,
                    exercise_type: ExerciseType::by_id(&exercise_entry.exercise_type_id)
                        .first(&conn.0)
                        .expect("Enable to get exercise type"),
                    exercise: exercise_entry,
                }
            } else {
                panic!("Encounterd Entry without corrisponding Entry type")
            }
        })
        .collect::<Vec<EntryContext>>();

    let context = IndexContext::new(
        entries,
        Food::all().get_results::<Food>(&conn.0)?,
        MealType::all().get_results::<MealType>(&conn.0)?,
        ExerciseType::all().get_results::<ExerciseType>(&conn.0)?,
    );
    Ok(Template::render("index", context))
}
