use std::sync::{atomic::AtomicBool, Arc};

use anyhow::{ensure, Error};

pub fn run_server() -> Result<(), Error> {
    let sigterm: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    ensure!(
        signal_hook::flag::register(signal_hook::consts::SIGTERM, sigterm.clone()).is_ok(),
        "Could not register sigterm"
    );
    
    #[allow(unused_must_use)]
    thread_priority::unix::set_current_thread_priority(thread_priority::ThreadPriority::Min);
    
    println!("Server started");
    while !sigterm.load(std::sync::atomic::Ordering::Relaxed) {
        println!("I live");
    }
    println!("Server stopped");

    return Ok(()); 
}
