use crate::schema::meal_entries;

#[derive(Insertable)]
#[table_name = "meal_entries"]
pub struct NewMealEntry {
    pub entry_id: i32,
    pub meal_type_id: i32,
}
