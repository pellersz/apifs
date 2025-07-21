#[macro_use]
extern crate cli_log;
use std::os::unix::process::CommandExt;
use std::process::{Command as SysCommand, exit};

use crate::command::Command;
use crate::data::Data;
use crate::exit_codes::*;
use crate::file_manipulation::{get_data, get_mainpath, get_program, update_data};
use crate::options::parse_options;
use crate::server::run_server;

mod apifs_object;
mod command;
mod data;
mod exit_codes;
mod file_manipulation;
mod media;
mod note;
mod options;
mod reminder;
mod server;

//TODO: function parameters to be coerible
fn main() {
    init_cli_log!();
    let args: Vec<String> = std::env::args().collect();
    match parse_options(args) {
        Ok(command) => handle_command(command),
        Err(err) => {
            error!("{err}");
            eprintln!("{err}");
            exit(ExitCode::WrongArguments as i32);
        }
    }
}

// TODO: make this more politically correct (do the command pattern)
fn handle_command(command: Command) {
    match command {
        Command::Help => {
            let mut main_path = get_mainpath();
            main_path.push("help.txt");
            let help_txt_res: Result<Vec<u8>, _> = std::fs::read(main_path);
            match help_txt_res {
                Ok(help_txt) => {
                    let decoded_help_txt = String::from_utf8(help_txt).unwrap();
                    info!("{decoded_help_txt}");
                    print!("{decoded_help_txt}");
                }
                Err(err) => {
                    error!("There was an error opening \"help.txt\": {err}");
                    eprintln!("There was an error opening \"help.txt\"");
                    exit(ExitCode::FileError as i32);
                }
            }
        }

        Command::Start => {
            let mut is_not_already_running: SysCommand =
                get_program("scripts/is_not_already_running.sh", None);

            if let Ok(exit_code) = is_not_already_running.status() {
                if !exit_code.success() {
                    error!("There is an apifs instance already running");
                    eprintln!("There is an apifs instance already running");
                    exit(ExitCode::AlreadyRunning as i32);
                }
            } else {
                error!("There was an error running a helper script");
                eprintln!("There was an error running a helper script");
                exit(ExitCode::FileError as i32);
            }

            match run_server() {
                Ok(_) => {}
                Err(err) => {
                    error!("{err}");
                    eprintln!("{err}");
                    exit(ExitCode::ServerRunError as i32)
                }
            }
        }

        Command::Stop => {
            let _ = get_program("scripts/stop_apifs.sh", None).exec();
            exit(ExitCode::FileError as i32);
        }

        Command::Remind(reminder) => match get_data() {
            Ok(mut data_file) => {
                data_file.reminders.push(reminder);
                let update_res = update_data(&data_file);
                if update_res.is_err() {
                    let update_res = update_data(&data_file);
                    let Err(err) = update_res else {
                        exit(ExitCode::Finished as i32);
                    };
                    error!("Could not update reminders: {err}");
                    eprintln!("Could not update reminders");
                    exit(ExitCode::FileError as i32);
                }
            }
            Err(err) => {
                error!("There was an error opening \"data.json\":{err}");
                eprintln!("There was an error opening \"data.json\"");
                exit(ExitCode::FileError as i32);
            }
        },

        Command::Note(note) => match get_data() {
            Ok(mut data_file) => {
                data_file.notes.push(note);
                let update_res = update_data(&data_file);
                if update_res.is_err() {
                    let update_res = update_data(&data_file);
                    let Err(err) = update_res else {
                        exit(ExitCode::Finished as i32);
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
        },

        Command::Show(data_to_show) => {
            let data = get_data().expect("Could not retrieve data");
            match data_to_show {
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

        Command::Delete(data_to_delete) => {
            let mut data = get_data().expect("Could not retrieve data");
            match data_to_delete {
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

        _ => {
            unimplemented!("Command not yet implemented");
        }
    }
}
