use cli_log::*;
use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}};
use thread_priority::unix::{set_current_thread_priority};

use crate::options::{parse_options, Command};

mod options;

fn main() {
    init_cli_log!();

    let args: Vec<String> = std::env::args().collect();
    let opt_command: Option<Command> = parse_options(args);    

    // TODO: change Option to Result 
    if let Some(command) = opt_command {
        match command {
            Command::Start => {
                let sigterm: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
                signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&sigterm))
                    .expect("An OS error occured:");

                #[allow(unused_must_use)]
                set_current_thread_priority(thread_priority::ThreadPriority::Min);

                println!("Started successfully");
                while !sigterm.load(Ordering::Relaxed) {
                    println!("Working"); 
                } 
                println!("Stopped successfully");
            },
            _     =>   {
                println!("Command not yet implemented");
            }
        }
    } else {
        return;
    }
    
                   
    
}
