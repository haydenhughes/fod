use chrono::NaiveDateTime;
use crate::schema::sleep_entries;

#[derive(Insertable)]
#[table_name = "sleep_entries"]
pub struct NewSleepEntry {
    pub entry_id: i32,
    pub duration: NaiveDateTime,
}
