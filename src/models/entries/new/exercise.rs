use crate::schema::exercise_entries;

#[derive(Insertable)]
#[table_name = "exercise_entries"]
pub struct NewExerciseEntry {
    pub entry_id: i32,
    pub exercise_type_id: i32,
    pub duration: NaiveDateTime,
}
