#[allow(unused_imports)]
use std::{os::unix::process::CommandExt, path::{Path, PathBuf}, process::Command as SysCommand, time::Duration};

#[test]
fn test() {
    let mut main_path = std::env::current_exe().unwrap();
    main_path.pop();
 
    let mut temp: PathBuf = Default::default();
    main_path.clone_into(&mut temp);
    temp.pop();
    temp.push("apifs");
    println!("{:?}", temp);
    SysCommand::new(&temp)
        .args(["start"].iter())
        .spawn()
        .expect("could not start server");

    main_path.clone_into(&mut temp);
    temp.push("scripts/is_not_already_running.sh");
    let mut is_not_already_running: SysCommand = SysCommand::new(&temp);
    if let Ok(exit_code) = is_not_already_running.status() {
        if exit_code.success() { 
            panic!("Server did not start, or stopped");
        }
    }
    
    main_path.clone_into(&mut temp);
    temp.pop();
    temp.push("apifs");
    SysCommand::new(&temp)
        .args(["stop"].iter())
        .status()
        .expect("could not start server");

    main_path.clone_into(&mut temp);
    temp.push("scripts/is_not_already_running.sh");
    let mut is_not_already_running: SysCommand = SysCommand::new(&temp);
    if let Ok(exit_code) = is_not_already_running.status() {
        if !exit_code.success() { 
            panic!("Server did not stop");
        }
    }
    
}
