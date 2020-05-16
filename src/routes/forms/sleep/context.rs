use crate::models::entries::new::{NewEntry, NewSleepEntry};
use crate::routes::forms::parse_timestamp;

#[derive(FromForm)]
pub struct NewSleepEntryForm {
    start_date: String,
    start_time: String,
    end_date: String,
    end_time: String,
    comments: Option<String>,
}
impl NewSleepEntryForm {
    pub fn as_entry(&self, user_id: i32) -> NewEntry {
        NewEntry {
            user_id,
            timestamp: parse_timestamp(&self.start_date, &self.start_time),
            comments: self.comments.to_owned(),
        }
    }

    pub fn as_sleep_entry(&self, entry_id: i32) -> NewSleepEntry {
        NewSleepEntry {
            entry_id,
            duration: parse_timestamp(&self.end_date, &self.end_time),
        }
    }
}
