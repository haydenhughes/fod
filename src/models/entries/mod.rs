mod exercise_entry;
mod meal_entry;
mod meta_entry;
pub mod new;
mod sleep_entry;

pub use exercise_entry::ExerciseEntry;
pub use meal_entry::MealEntry;
pub use meta_entry::MetaEntry;
pub use sleep_entry::SleepEntry;

pub enum Entry {
    Sleep(MetaEntry, SleepEntry),
    Meal(MetaEntry, MealEntry),
    Exercise(MetaEntry, ExerciseEntry),
}
