use crate::{
    command::Command,
    exit_codes::ExitCode,
    file_manipulation::{get_data, get_program, update_data},
    reminder::Reminder,
};
use std::process::exit;

pub struct Remind {
    reminder: Reminder,
}

impl Remind {
    pub fn new(reminder: Reminder) -> Self {
        Remind { reminder }
    }
}

impl Command for Remind {
    fn execute(&self) {
        match get_data() {
            Ok(mut data_file) => {
                data_file.reminders.push(self.reminder.clone());
                let update_res = update_data(&data_file);
                if update_res.is_err() {
                    let update_res = update_data(&data_file);
                    let Err(err) = update_res else {
                        let _ = get_program("scripts/signal_to_server.sh", None).spawn();
                        exit(ExitCode::Finished as i32);
                    };
                    error!("Could not update reminders: {err}");
                    eprintln!("Could not update reminders");
                    exit(ExitCode::FileError as i32);
                }
                let _ = get_program("scripts/signal_to_server.sh", None).spawn();
            }
            Err(err) => {
                error!("There was an error opening \"data.json\":{err}");
                eprintln!("There was an error opening \"data.json\"");
                exit(ExitCode::FileError as i32);
            }
        }
    }
}
