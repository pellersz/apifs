use crate::{
    apifs_object::ApifsObject, command::Command, data::Data, exit_codes::ExitCode,
    file_manipulation::get_data,
};
use std::process::exit;

pub struct Show {
    data_to_show: Data,
}

impl Show {
    pub fn new(data_to_show: Data) -> Self {
        Show { data_to_show }
    }
}

impl Command for Show {
    fn execute(&self) {
        let data: ApifsObject = get_data().expect("Could not retrieve data");
        match self.data_to_show.clone() {
            Data::All => println!("{data}"),
            Data::Reminder(id) => {
                if id < data.reminders.len() {
                    println!("{}", data.reminders[id]);
                    return;
                }
                error!("No such reminder: {id}");
                eprintln!("No such reminder.");
                exit(ExitCode::ResourceNotFound as i32)
            }
            Data::Note(name) => {
                if let Some(note) = data.notes.iter().find(|note| note.name == name) {
                    println!("{note}");
                    return;
                }
                error!("No such note: {name}");
                eprintln!("No such note.");
                exit(ExitCode::ResourceNotFound as i32);
            }
        }
    }
}
