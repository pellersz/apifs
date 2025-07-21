use anyhow::{Error, bail, ensure};
use file_lock::{FileLock, FileOptions};
use lazy_static::lazy_static;
use serde_json::{from_reader, to_writer_pretty};
use std::{path::PathBuf, process::Command};

use crate::apifs_object::ApifsObject;

lazy_static! {
    static ref MAIN_PATH: PathBuf = {
        let main_path_res = std::env::current_exe();
        if main_path_res.is_err() {
            panic!("Could not get path to program");
        }
        let mut path = main_path_res.unwrap();
        path.pop();
        path
    };
}

pub fn get_mainpath() -> PathBuf {
    return MAIN_PATH.clone();
}

pub fn get_data() -> Result<ApifsObject, Error> {
    let mut main_path = get_mainpath();

    main_path.push("data.json");

    match FileLock::lock(&main_path, true, FileOptions::new().read(true)) {
        Ok(data_file) => {
            let file_contents_res: Result<ApifsObject, serde_json::Error> =
                from_reader(&data_file.file);
            let _ = data_file.unlock();
            ensure!(
                file_contents_res.is_ok(),
                "There was an error reading apifs data"
            );
            Ok(file_contents_res.unwrap())
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => Ok(Default::default()),
            _ => {
                bail!("Could not open \"data.json\", but it exists");
            }
        },
    }
}

pub fn update_data(object: &ApifsObject) -> Result<(), Error> {
    let mut main_path = get_mainpath();
    main_path.push("data.json");
    match FileLock::lock(
        &main_path,
        true,
        FileOptions::new().write(true).truncate(true),
    ) {
        Ok(data_file) => {
            let write_res = to_writer_pretty(&data_file.file, object);
            let _ = data_file.unlock();
            ensure!(write_res.is_ok(), "There was an error reading apifs data");
            Ok(())
        }
        Err(_) => {
            bail!("There was an error opening \"data.json\"");
        }
    }
}

pub fn get_program(path: &str, args: Option<Vec<&str>>) -> Command {
    let mut main_path = get_mainpath();
    main_path.push(path);
    let mut res = Command::new(main_path);
    if let Some(act_args) = args {
        res.args(act_args);
    }
    res
}
