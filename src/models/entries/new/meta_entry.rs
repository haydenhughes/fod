use crate::schema::meta_entries;
use chrono::NaiveDateTime;

#[derive(Insertable)]
#[table_name = "meta_entries"]
pub struct NewMetaEntry {
    pub user_id: i32,
    pub timestamp: NaiveDateTime,
    pub comments: Option<String>,
}
