use crate::schema::meal_types;

#[derive(Insertable)]
#[table_name = "meal_types"]
pub struct NewMealType {
    pub name: String,
}
