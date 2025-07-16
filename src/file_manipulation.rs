use anyhow::{ensure, Error};
use serde_json::to_writer_pretty;
use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf, process::Command};

use crate::apifs_object::ApifsObject;

fn get_mainpath() -> PathBuf {
    static mut MAIN_PATH: Option<PathBuf> = None;
    unsafe {
        if MAIN_PATH.is_none() {
            let main_path_res = std::env::current_exe();
            if main_path_res.is_err() { panic!("Could not get path to program"); }
            MAIN_PATH = Some(main_path_res.unwrap());
        }
        return MAIN_PATH.clone().unwrap();
    } 
}

pub fn get_data() -> Result<ApifsObject, Error> {
    let mut main_path = get_mainpath();     
    main_path.push("data.json");
    let data_file_res: Result<File, _> = File::open(&main_path);
    ensure!(data_file_res.is_ok(), "There was an error opening \"data.json\"");
    
    let data_file = data_file_res.unwrap();
    if BufReader::new(&data_file).lines().next().is_none() { return Ok(Default::default()); }
 
    let file_contents_res: Result<ApifsObject, serde_json::Error> = serde_json::from_reader(data_file);
    ensure!(file_contents_res.is_ok(), "There was an error reading apifs data");
    Ok(file_contents_res.unwrap())
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
