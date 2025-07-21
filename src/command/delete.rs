use crate::{
    command::Command,
    data::Data,
    exit_codes::ExitCode,
    file_manipulation::{get_data, update_data},
};
use std::process::exit;

pub struct Delete {
    data_to_delete: Data,
}

impl Delete {
    pub fn new(data_to_delete: Data) -> Self {
        Delete { data_to_delete }
    }
}

impl Command for Delete {
    fn execute(&self) {
        let mut data = get_data().expect("Could not retrieve data");
        match self.data_to_delete.clone() {
            Data::Reminder(id) => {
                if id < data.reminders.len() {
                    data.reminders.remove(id);
                } else {
                    error!("No such reminder: {id}");
                    eprintln!("No such note.");
                    exit(ExitCode::ResourceNotFound as i32);
                }
            }
            Data::Note(name) => {
                let mut found = false;
                for i in 0..data.notes.len() {
                    if data.notes[i].name == name {
                        data.notes.remove(i);
                        found = true;
                    }
                }

                if !found {
                    error!("No such note: {name}");
                    eprintln!("No such note.");
                    exit(ExitCode::ResourceNotFound as i32);
                }
            }
            _ => exit(ExitCode::WrongArguments as i32),
        }
        if update_data(&data).is_err() {
            error!("Could not update data");
            eprintln!("Could not update data");
            exit(ExitCode::FileError as i32);
        }
    }
}
