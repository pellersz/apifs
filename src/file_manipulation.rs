use anyhow::{bail, ensure, Error};
use serde_json::{from_reader, to_writer_pretty};
use lazy_static::lazy_static;
use std::{fs::File, path::PathBuf, process::Command};

use crate::apifs_object::ApifsObject;

lazy_static! {
    static ref MAIN_PATH: PathBuf = { 
        let main_path_res = std::env::current_exe();
        if main_path_res.is_err() { panic!("Could not get path to program"); } 
        let mut path = main_path_res.unwrap();
        path.pop();
        path
    };
}

pub fn get_mainpath() -> PathBuf {
    return MAIN_PATH.clone();
}

// TODO: do these with locks, thank you
pub fn get_data() -> Result<ApifsObject, Error> {
    let mut main_path = get_mainpath();     

    main_path.push("data.json");

    let data_file_res: Result<File, _> = File::open(&main_path);
    match data_file_res {
        Ok(data_file) => {
            let file_contents_res: Result<ApifsObject, serde_json::Error> = from_reader(data_file);
            ensure!(file_contents_res.is_ok(), "There was an error reading apifs data");
            Ok(file_contents_res.unwrap())
        },
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::NotFound => {
                    Ok(Default::default())
                }, 
                _ => { bail!("Could not open \"data.json\", but it exists"); }
            }
        }
    }
}

pub fn update_data(object: &ApifsObject) -> Result<(), Error> {
    let mut main_path = get_mainpath();
    main_path.push("data.json");
    let data_file_res: Result<File, _> = File::create(&main_path);
    ensure!(data_file_res.is_ok(), "There was an error opening \"data.json\"");
    
    let data_file = data_file_res.unwrap();
 
    let write_res = to_writer_pretty(data_file, object);
    ensure!(write_res.is_ok(), "There was an error reading apifs data");
    Ok(())
}

pub fn get_program(path: &str) -> Command {
    let mut main_path = get_mainpath();
    main_path.push(path);
    Command::new(main_path)
}

