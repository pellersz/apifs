use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration as TimeDuration;

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4::Picture;
use gtk4::glib::clone;
use gtk4::glib::ffi::g_timeout_add;
use gtk4::glib::property::PropertySet;
use gtk4::glib::{ControlFlow, timeout_add};
use gtk4::{self as gtk, Box, Button, Label};

use crate::file_manipulation::get_mainpath;
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
        //clone! macro not working on exit for some reason, or I just messed it up
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
            container.append(&Picture::for_filename(picture_path));
        }
        container.append(&description_label);
        container.append(&button);

        window.set_child(Some(&container));
        window.present();

        if let Some(ref sound_path) = sound_clone {
            // TODO: extend get_program to accept arguments too
            let mut play = get_mainpath();
            play.push("scripts/play_sound.sh");
            if let Ok(mut child) = Command::new(&play).arg(sound_path).spawn() {
                timeout_add(TimeDuration::from_millis(100), move || {
                    if exited.load(Ordering::Relaxed) {
                        let _ = child.kill();
                        return ControlFlow::Break;
                    }
                    return ControlFlow::Continue;
                });
            } else {
                let _ = Command::new(&play).arg(sound_path).spawn();
            }
        }
    });

    let app_args: [&str; 0] = [];
    app.run_with_args(&app_args);
}
