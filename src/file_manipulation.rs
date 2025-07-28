use anyhow::{Error, bail, ensure};
use file_lock::{FileLock, FileOptions};
use lazy_static::lazy_static;
use serde_json::{from_reader, to_writer_pretty};
use std::{fs::create_dir_all, path::PathBuf, process::Command};

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

fn get_data_path() -> PathBuf {
    let mut main_path: PathBuf;
    #[cfg(debug_assertions)]
    {
        main_path = get_mainpath();
    }
    #[cfg(not(debug_assertions))]
    {
        use std::env::{home_dir, var};
        use std::path::Path;

        match var("XDG_CONFIG_HOME") {
            Ok(conf_home) => {
                main_path = Path::new(&conf_home).to_path_buf();
            }
            _ => {
                main_path = home_dir().expect("Could not open home directory");
                main_path.push(".config");
            }
        }
        main_path.push("apifs");
    }
    main_path.push("data.json");
    main_path
}

pub fn get_data() -> Result<ApifsObject, Error> {
    let mut main_path = get_data_path();
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
            std::io::ErrorKind::NotFound => {
                main_path.pop();
                if create_dir_all(main_path).is_err() {
                    eprintln!(
                        "Could not create $XDG_CONFIG_HOME/apifs directory or ~/.config/apifs directory"
                    );
                    warn!(
                        "Could not create $XDG_CONFIG_HOME/apifs directory or ~/.config/apifs directory"
                    );
                }
                Ok(Default::default())
            }
            _ => {
                bail!("Could not open \"data.json\", but it exists");
            }
        },
    }
}

pub fn update_data(object: &ApifsObject) -> Result<(), Error> {
    let main_path = get_data_path();
    match FileLock::lock(
        &main_path,
        true,
        FileOptions::new().write(true).truncate(true).create(true),
    ) {
        Ok(data_file) => {
            let write_res = to_writer_pretty(&data_file.file, object);
            let _ = data_file.unlock();
            ensure!(write_res.is_ok(), "There was an error reading apifs data");
            Ok(())
        }
        _ => {
            bail!("There was an error opening \"data.json\"");
        }
    }
}

pub fn get_program(path: &str, args: Option<Vec<&str>>) -> Command {
    let mut main_path = get_mainpath();

    #[cfg(not(debug_assertions))]
    main_path.push("../../opt/apifs/");

    main_path.push(path);
    let mut res = Command::new(main_path);
    if let Some(act_args) = args {
        res.args(act_args);
    }
    res
}
