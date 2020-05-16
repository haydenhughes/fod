use crate::schema::entries;
use chrono::NaiveDateTime;

#[derive(Insertable)]
#[table_name = "entries"]
pub struct NewEntry {
    pub user_id: i32,
    pub timestamp: NaiveDateTime,
    pub comments: Option<String>,
}
