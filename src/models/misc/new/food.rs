use crate::schema::foods;

#[derive(FromForm, Insertable)]
#[table_name = "foods"]
pub struct NewFood {
    pub name: String,
}
