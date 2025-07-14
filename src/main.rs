#[macro_use] extern crate cli_log;
use std::os::unix::process::CommandExt;
//use cli_log::*;
use std::process::{exit, Command as SysCommand};

use crate::options::{parse_options, Command};
use crate::server::run_server;
use crate::exit_codes::*;

mod options;
mod server;
mod exit_codes;

fn main() {
    init_cli_log!();

    let args: Vec<String> = std::env::args().collect();
    match parse_options(args) {
        Ok(command) => {
            match std::env::current_exe() {
                Ok(mut main_path) => {
                    main_path.pop();
                    match command {
                        Command::Help => {
                            main_path.push("help.txt");
                            println!("{:?}", main_path);
                            let help_txt_res: Result<Vec<u8>, _> = std::fs::read(main_path);
                            match help_txt_res {
                                Ok(help_txt) => {
                                    let decoded_help_txt = String::from_utf8(help_txt).unwrap(); 
                                    info!("{decoded_help_txt}");
                                    print!("{decoded_help_txt}");
                                },
                                Err(_) => {
                                    error!("There was an error opening \"help.txt\"");
                                    println!("There was an error opening \"help.txt\"");
                                    exit(ExitCode::ScriptIssue as i32);
                                }
                            }
                        }

                        Command::Start => { 
                            main_path.push("scripts/is_not_already_running.sh"); 
                            let mut is_not_already_running: SysCommand = SysCommand::new(main_path);

                            if let Ok(exit_code) = is_not_already_running.status() {
                                if !exit_code.success() {
                                    error!("There is an apifs instance already running");
                                    eprintln!("There is an apifs instance already running");
                                    exit(ExitCode::AlreadyRunning as i32);
                                }
                            } else {
                                error!("There was an error running a helper script");
                                eprintln!("There was an error running a helper script");
                                exit(ExitCode::ScriptIssue as i32);
                            }

                            match run_server() {
                                Ok(_) => {},
                                Err(err) => {
                                    error!("{err}");
                                    eprintln!("{err}");
                                    exit(ExitCode::ServerRunError as i32)
                                }
                            }
                        },

                        Command::Stop => {
                            main_path.push("scripts/stop_apifs.sh");
                            #[allow(unused_must_use)]
                            SysCommand::new(main_path).exec();    
                        },

                        _     =>   {
                            unimplemented!("Command not yet implemented");
                        }
                    }
                }, 

                Err(err) => {
                    error!("{err}");
                    eprintln!("{err}");
                    exit(ExitCode::PathError as i32);
                }
            }
        },

        Err(err) => {
            error!("{err}");
            eprintln!("{err}");
            exit(ExitCode::WrongArguments as i32);
        }
    } 
}
