use std::{sync::{atomic::AtomicBool, Arc}, time::Duration};

use anyhow::{ensure, Error};

use crate::file_manipulation::get_data;

pub fn run_server() -> Result<(), Error> {
    let sigterm: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    ensure!(
        signal_hook::flag::register(signal_hook::consts::SIGTERM, sigterm.clone()).is_ok(),
        "Could not register sigterm"
    );
    
    #[allow(unused_must_use)]
    thread_priority::unix::set_current_thread_priority(thread_priority::ThreadPriority::Min);
 
    let data_res = get_data();
    match data_res {
        Ok(data) => {
            println!("Server started");
            while !sigterm.load(std::sync::atomic::Ordering::Relaxed) {
                 std::thread::sleep(Duration::from_secs(1));
            }
            println!("Server stopped");
            Ok(())
        }, 
        Err(err) => Err(err)
    }
}
