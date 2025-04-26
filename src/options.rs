use chrono::NaiveDate;

enum ReminderType {
    Once,
    Daily,
    SpecificInterval,
    Specific,
}

struct RemindOnceOptions {
    date: NaiveDate,
    sound: Option<String>,
    picture: Option<String>,
}

struct NoteOptions {
    text: String
}

