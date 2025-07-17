use crate::{data::Data, note::Note, reminder::Reminder};

pub enum Command {
    Remind(Reminder),
    Note(Note),
    Start,
    Stop,
    Help,
    AddMedia(String, String),
    Delete(String),
    Show(Data),
    NoCommand
}
