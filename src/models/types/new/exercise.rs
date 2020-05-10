use crate::schema::exercise_types;

#[derive(FromForm, Insertable)]
#[table_name = "exercise_types"]
pub struct NewExerciseType {
    pub name: String,
}
