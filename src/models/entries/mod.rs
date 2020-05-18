mod meta;
mod exercise;
mod meal;
pub mod new;
mod sleep;

pub use meta::MetaEntry;
pub use exercise::ExerciseEntry;
pub use meal::MealEntry;
pub use sleep::SleepEntry;

pub enum Entry {
    Sleep(MetaEntry, SleepEntry),
    Meal(MetaEntry, MealEntry),
    Exercise(MetaEntry, ExerciseEntry),
}
