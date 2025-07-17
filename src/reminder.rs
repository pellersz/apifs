use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Weekday};

use crate::media::Media;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Reminder {
    Once(NaiveDateTime, Media, Option<String>),
    // MoreThanOnce(Vec<NaiveDateTime>, Media, Option<String>),
    // expl: 
    // param1: time of day to remind 
    // param2: days of the week to notify
    // param3: last day when the remind happened
    Daily(NaiveTime, [bool; 7], NaiveDate, Media, Option<String>),
    SpecificInterval(NaiveDateTime, Duration, Media, Option<String>),
}

