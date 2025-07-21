use crate::{command::Command, exit_codes::ExitCode, file_manipulation::get_program};
use std::{os::unix::process::CommandExt, process::exit};

pub struct Stop {}

impl Command for Stop {
    fn execute(&self) {
        let _ = get_program("scripts/stop_apifs.sh", None).exec();
        exit(ExitCode::FileError as i32);
    }
}
