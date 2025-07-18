use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4::{self as gtk, Button, Label};

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

        println!("{:?} {:?}", sound_clone, picture_clone);
        let description_label = Label::new(Option::as_deref(&description_clone));
        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.set_child(Some(&button));
        window.set_child(Some(&description_label));

        // Show the window.
        window.present();
    });

    let app_args: [&str; 0] = [];
    app.run_with_args(&app_args);
}
