use crate::models::types::{ExerciseType, MealType};
use crate::models::misc::Food;
use crate::models::entries::{Entry, ExerciseEntry, SleepEntry};
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum EntryContext<'a> {
    Meal {
        entry: &'a Entry,
        meal_type: MealType,
        foods: Vec<Food>,
    },
    Sleep {
        entry: &'a Entry,
        sleep: SleepEntry,
    },
    Exercise {
        entry: &'a Entry,
        exercise: ExerciseEntry,
        exercise_type: ExerciseType,
    },
}

#[derive(Serialize)]
pub struct IndexContext<'a> {
    pub entries: Vec<EntryContext<'a>>,
    pub foods: Vec<Food>,
    pub meal_types: Vec<MealType>,
    pub exercise_types: Vec<ExerciseType>,
}

impl<'a> IndexContext<'a> {
    pub fn new(
        entries: Vec<EntryContext<'a>>,
        foods: Vec<Food>,
        meal_types: Vec<MealType>,
        exercise_types: Vec<ExerciseType>,
    ) -> Self {
        IndexContext {
            entries,
            foods,
            meal_types,
            exercise_types,
        }
    }
}
