use serde::{Deserialize, Serialize};

use crate::{options::Note, reminder::Reminder};

#[derive(Default, Deserialize, Serialize)]
pub struct ApifsObject {
    reminders: Vec<Reminder>,
    notes: Vec<Note>
}
