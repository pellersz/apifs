#[macro_use] extern crate cli_log;
//use cli_log::*;
use std::{path::Path, process::{exit, Command as SysCommand}};

use crate::options::{parse_options, Command};
use crate::server::run_server;
use crate::exit_codes::*;

mod options;
mod server;
mod exit_codes;

fn main() {
    init_cli_log!();

    let args: Vec<String> = std::env::args().collect();

    // TODO: change Option to Result 
    if let Some(command) = parse_options(args) {
        if let Ok(mut main_path) = std::env::current_exe() {
            match command {
                Command::Start => {
                    main_path.push("scripts/is_not_already_running.sh");
                    let mut is_not_already_running: SysCommand = SysCommand::new(main_path);

                    if let Ok(exit_code) = is_not_already_running.status() {
                        if !exit_code.success() {
                            error!("There is an apifs instance already running");
                            exit(ExitCode::AlreadyRunning as i32);
                        }
                    } else {
                        error!("There was an error running a helper script");
                        exit(ExitCode::ScriptIssue as i32);
                    }

                    match run_server() {
                        Ok(_) => {},
                        Err(err) => {
                            error!("{err}");
                            exit(ExitCode::ServerRunError as i32)
                        }
                    }
                },
                Command::Stop => {
                    main_path.push("scripts/stop_apifs.sh");
                    SysCommand::new(main_path);    
                },
                _     =>   {
                    println!("Command not yet implemented");
                }
            }
        }
    } else {
        exit(1);
    }
}
