use serde::{Deserialize, Serialize};

use crate::{options::Note, reminder::Reminder};

#[derive(Default, Deserialize, Serialize)]
pub struct ApifsObject {
    pub reminders: Vec<Reminder>,
    pub notes: Vec<Note>
}
