use crate::{command::Command, exit_codes::ExitCode, file_manipulation::get_mainpath};
use std::process::exit;

pub struct Help {}

impl Command for Help {
    fn execute(&self) {
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
}
