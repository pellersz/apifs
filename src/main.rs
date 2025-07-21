#[macro_use]
extern crate cli_log;

use std::process::exit;

use crate::command::parse_command;
use crate::exit_codes::*;

mod apifs_object;
mod command;
mod data;
mod exit_codes;
mod file_manipulation;
mod media;
mod note;
mod reminder;
mod server;

fn main() {
    init_cli_log!();
    let args: Vec<String> = std::env::args().collect();
    match parse_command(args) {
        Ok(command) => {
            command.execute();
        }
        Err(err) => {
            error!("{err}");
            eprintln!("{err}");
            exit(ExitCode::WrongArguments as i32);
        }
    }
}
