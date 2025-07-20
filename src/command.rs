use crate::{data::Data, note::Note, reminder::Reminder};

// TODO: possibly move this to options and maybe rename that to command.rs
pub enum Command {
    Remind(Reminder),
    Note(Note),
    Start,
    Stop,
    Help,
    AddSound(String, String),
    AddPicture(String, String),
    Delete(String),
    Show(Data),
    NoCommand,
}
