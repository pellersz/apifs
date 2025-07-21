use crate::{
    command::Command,
    exit_codes::ExitCode,
    file_manipulation::{get_data, get_program, update_data},
    note::Note,
};
use std::{os::unix::process::CommandExt, process::exit};

pub struct WriteNote {
    note: Note,
}

impl WriteNote {
    pub fn new(note: Note) -> Self {
        WriteNote { note }
    }
}

impl Command for WriteNote {
    fn execute(&self) {
        match get_data() {
            Ok(mut data_file) => {
                data_file.notes.push(self.note.clone());
                let update_res = update_data(&data_file);
                if update_res.is_err() {
                    let update_res = update_data(&data_file);
                    let Err(err) = update_res else {
                        let _ = get_program("scripts/signal_to_server.sh", None).exec();
                        exit(ExitCode::FileError as i32);
                    };
                    error!("Could not update notes: {err}");
                    eprintln!("Could not update notes");
                    exit(ExitCode::FileError as i32);
                }
            }
            Err(err) => {
                error!("There was an error opening \"data.json\":{err}");
                eprintln!("There was an error opening \"data.json\"");
                exit(ExitCode::FileError as i32);
            }
        }
    }
}
