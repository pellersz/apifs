use chrono::{Duration, NaiveDateTime, NaiveTime};

use crate::media::Media;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Reminder {
    Once(NaiveDateTime, Media, Option<String>),
    // MoreThanOnce(Vec<NaiveDateTime>, Media, Option<String>),
    Daily(NaiveTime, [bool; 7], Media, Option<String>),
    SpecificInterval(NaiveDateTime, Duration, Media, Option<String>),
}

