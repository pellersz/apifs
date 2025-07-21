use std::fmt::Display;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration as TimeDuration;

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4::Picture;
use gtk4::glib::clone;
use gtk4::glib::property::PropertySet;
use gtk4::glib::{ControlFlow, timeout_add};
use gtk4::{self as gtk, Box, Button, Label};

use crate::file_manipulation::get_mainpath;
use crate::file_manipulation::get_program;
use crate::media::Media;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Reminder {
    Once(NaiveDateTime, Media, Option<String>),
    // MoreThanOnce(Vec<NaiveDateTime>, Media, Option<String>),
    // expl:
    // param1: time of day to remind
    // param2: days of the week to notify
    // param3: last day when the remind happened
    Daily(NaiveTime, [bool; 7], NaiveDate, Media, Option<String>),
    SpecificInterval(NaiveDateTime, Duration, Media, Option<String>),
}

fn print_misc(f: &mut std::fmt::Formatter<'_>, media: &Media, description: &Option<String>) {
    if let Some(picture) = &media.picture {
        let _ = write!(f, ", with picture {picture}");
    }
    if let Some(sound) = &media.sound {
        let _ = write!(f, ", with sound {sound}");
    }
    if let Some(description_text) = description {
        let _ = write!(f, ", description: {description_text}");
    }
}

impl Display for Reminder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            &Reminder::Once(datetime, media, description) => {
                let _ = write!(f, "Remind on {datetime}");
                print_misc(f, media, description);
            }
            &Reminder::Daily(time, days, _last_day_notified, media, description) => {
                let _ = write!(f, "Remind at {time} on ");
                static DAY_NAMES: [&str; 7] = [
                    "Monday",
                    "Tuesday",
                    "Wednesday",
                    "Thursday",
                    "Friday",
                    "Saturday",
                    "Sunday",
                ];
                for i in 1..7 {
                    if days[i] {
                        let _ = write!(f, "{},", DAY_NAMES[i]);
                    }
                }
                let _ = write!(f, "{}", 8u8 as char);
                print_misc(f, media, description);
            }
            &Reminder::SpecificInterval(datetime, interval, media, description) => {
                let _ = write!(f, "From {datetime}, remind every {interval}");
                print_misc(f, media, description);
            }
        }
        Ok(())
    }
}

// This is not how I imagined, I don't know how to open multiple windows, but it works for one
pub fn display_reminder(
    description: &Option<String>,
    sound: &Option<String>,
    picture: &Option<String>,
) {
    let app = Application::builder()
        .application_id("apifs.reminder")
        .build();

    let sound_clone = sound.clone();
    let picture_clone = picture.clone();
    let description_clone = description.clone();

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Reminder!")
            .build();

        let container = Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .build();

        let description_label = Label::new(Option::as_deref(&description_clone));

        let exited = Arc::new(AtomicBool::new(false));

        let button = Button::with_label("Ok");
        //clone! macro not working on exit for some reason
        let exited_ref = exited.clone();
        button.connect_clicked(clone!(
            #[strong]
            window,
            move |_| {
                exited_ref.set(true);
                window.close();
            }
        ));

        if let Some(picture_path) = &picture_clone {
            container.append(&Picture::for_filename(
                String::from(get_mainpath().to_str().unwrap()) + "/" + picture_path,
            ));
        }
        container.append(&description_label);
        container.append(&button);

        window.set_child(Some(&container));
        window.present();

        if let Some(ref sound_path) = sound_clone {
            let mut play = get_program(
                "scripts/play_sound.sh",
                Some(vec![
                    (String::from(get_mainpath().to_str().unwrap()) + "/" + sound_path).as_str(),
                ]),
            );
            if let Ok(mut child) = play.spawn() {
                timeout_add(TimeDuration::from_millis(100), move || {
                    if exited.load(Ordering::Relaxed) {
                        let _ = child.kill();
                        return ControlFlow::Break;
                    }
                    return ControlFlow::Continue;
                });
            } else {
                warn!("Could not play file");
            }
        }
    });

    let app_args: [&str; 0] = [];
    app.run_with_args(&app_args);
}
