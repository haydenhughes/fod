pub mod sleep;

use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

pub fn parse_timestamp<S: AsRef<str>>(date: S, time: S) -> NaiveDateTime {
    NaiveDateTime::new(
        NaiveDate::parse_from_str(date.as_ref(), "%Y-%m-%d").expect("Error parsing date"),
        NaiveTime::parse_from_str(time.as_ref(), "%H:%M").expect("Error parsing time"),
    )
}
