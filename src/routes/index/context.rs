use crate::models::entries::{Entry, ExerciseEntry, SleepEntry};
use crate::models::misc::Food;
use crate::models::types::{ExerciseType, MealType};
use chrono::NaiveDateTime;
use serde::Serialize;

fn format_date(s: NaiveDateTime) -> String {
    s.format("%B %m, %Y").to_string()
}

fn format_time(s: NaiveDateTime) -> String {
    s.format("%H:%M").to_string()
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum EntryContext {
    Sleep {
        id: i32,
        duration: String,
        start_date: String,
        start_time: String,
        end_date: String,
        end_time: String,
        comments: Option<String>,
    },
}

impl From<Entry> for EntryContext {
    fn from(e: Entry) -> Self {
        match e {
            Entry::Sleep(m, s) => Self::Sleep {
                id: m.id,
                duration: (s.duration - m.timestamp).to_string(),
                start_date: format_date(m.timestamp),
                start_time: format_time(m.timestamp),
                end_date: format_date(s.duration),
                end_time: format_time(s.duration),
                comments: m.comments,
            },
        }
    }
}

#[derive(Serialize)]
pub struct IndexContext {
    pub entries: Vec<EntryContext>,
    pub foods: Vec<Food>,
    pub meal_types: Vec<MealType>,
    pub exercise_types: Vec<ExerciseType>,
}

impl<'a> IndexContext {
    pub fn new(
        entries: Vec<EntryContext>,
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
