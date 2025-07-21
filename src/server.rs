use anyhow::{Error, ensure};
use chrono::{Datelike, Local};
use signal_hook::{
    consts::{SIGTERM, SIGUSR1},
    flag,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::Duration,
};
use thread_priority::{ThreadPriority, unix::set_current_thread_priority};

use crate::{
    file_manipulation::{get_data, update_data},
    media::Media,
    reminder::{Reminder, display_reminder},
};

pub fn run_server() -> Result<(), Error> {
    let sigterm = Arc::new(AtomicBool::new(false));
    let sigupdate = Arc::new(AtomicBool::new(false));
    ensure!(
        flag::register(SIGTERM, sigterm.clone()).is_ok()
            && flag::register(SIGUSR1, sigupdate.clone()).is_ok(),
        "Could not register sigterm"
    );

    let _ = set_current_thread_priority(ThreadPriority::Min);

    let data_res = get_data();
    match data_res {
        Ok(mut data) => {
            println!("Server started");
            while !sigterm.load(Ordering::Relaxed) {
                if sigupdate.load(Ordering::Relaxed) {
                    let data_res = get_data();
                    match data_res {
                        Ok(new_data) => {
                            data.reminders = new_data.reminders;
                        }
                        Err(err) => {
                            warn!("Could not update reminder data{err}");
                        }
                    }
                }

                let reminders = &mut data.reminders;
                let mut to_remove: Vec<usize> = Vec::new();
                let mut changed = false;
                for i in 0..reminders.len() {
                    match &reminders[i] {
                        Reminder::Once(datetime, media, description) => {
                            if Local::now().naive_local() >= *datetime {
                                notify(description, media);
                                changed = true;
                                to_remove.push(i);
                            }
                        }
                        Reminder::Daily(time, days, last_day, media, description) => {
                            let datetime = Local::now().naive_local();
                            if datetime.time() >= *time
                                && datetime.date() != *last_day
                                && days[datetime.weekday() as usize]
                            {
                                notify(description, media);
                                changed = true;
                                // This might not be the best way, maybe i should have put some
                                // struct here to achieve updating a value, idk, possibly fix in
                                // future? (similar with specific interval)
                                reminders[i] = Reminder::Daily(
                                    *time,
                                    *days,
                                    datetime.date(),
                                    media.clone(),
                                    description.clone(),
                                );
                            }
                        }
                        Reminder::SpecificInterval(datetime, duration, media, description) => {
                            if Local::now().naive_local() >= *datetime {
                                notify(description, media);
                                changed = true;
                                reminders[i] = Reminder::SpecificInterval(
                                    *datetime + *duration,
                                    *duration,
                                    media.clone(),
                                    description.clone(),
                                );
                            }
                        }
                    }
                }
                if changed {
                    for i in to_remove.iter().rev() {
                        reminders.remove(*i);
                    }
                    match update_data(&data) {
                        Ok(_) => {}
                        Err(err) => {
                            error!("Could not update data: {err}");
                        }
                    }
                }
                sleep(Duration::from_secs(1));
            }
            println!("Server stopped");
            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn notify(description: &Option<String>, media: &Media) {
    display_reminder(description, &media.sound, &media.picture);
}
