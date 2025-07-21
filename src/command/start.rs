use crate::{
    command::Command, exit_codes::ExitCode, file_manipulation::get_program, server::run_server,
};
use std::process::{Command as SysCommand, exit};

pub struct Start {}

impl Command for Start {
    fn execute(&self) {
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
}
