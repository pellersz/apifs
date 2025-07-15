use chrono::{Duration, NaiveDateTime, NaiveTime};

use crate::media::Media;

#[derive(serde::Serialize)]
pub enum Reminder {
    Once(NaiveDateTime, Media, Option<String>),
    // MoreThanOnce(Vec<NaiveDateTime>, Media, Option<String>),
    Daily(NaiveTie, [bool; 7], Media, Option<String>),
    SpecificInterval(NaiveDateTime, Duration, Media, Option<String>),
}

