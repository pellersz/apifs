use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{note::Note, reminder::Reminder};

#[derive(Default, Deserialize, Serialize)]
pub struct ApifsObject {
    pub reminders: Vec<Reminder>,
    pub notes: Vec<Note>,
}

impl Display for ApifsObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "Reminders:");
        for reminder in &self.reminders {
            let _ = writeln!(f, "{reminder}");
        }
        println!("\nNotes:");
        for note in &self.notes {
            let _ = writeln!(f, "{note}");
        }
        Ok(())
    }
}
