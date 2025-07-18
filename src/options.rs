use anyhow::{Error, bail, ensure};
use chrono::Local;
use chrono::{Duration, NaiveDateTime, NaiveTime, TimeDelta};
use regex::Regex;
use std::{cmp, format};

use crate::command::Command;
use crate::media::Media;
use crate::note::Note;
use crate::reminder::Reminder;

// TODO: if you are bored, you can try to throw away this garbaggio and use clap
pub fn parse_options(args: Vec<String>) -> Result<Command, Error> {
    ensure!(args.len() >= 2, "Too few arguments");

    match args[1].as_str() {
        "start" => Ok(Command::Start),
        "stop" => Ok(Command::Stop),
        "notify" => {
            ensure!(args.len() >= 3, "Notify command needs a type!");

            let mut final_datetime: Option<NaiveDateTime> = None;
            let mut final_time: Option<NaiveTime> = None;
            let mut picture: Option<String> = None;
            let mut sound: Option<String> = None;
            let mut description: Option<String> = None;
            let mut i: usize = 3;

            match args[2].as_str() {
                "once" => {
                    while i < args.len() {
                        ensure!(
                            args.len() != i + 1,
                            "{} was not provided with a parameter!",
                            args[i]
                        );
                        match args[i].as_str() {
                            "-w" => {
                                let datetime = parse_datetime(&args, i + 1);
                                ensure!(
                                    datetime != None,
                                    "{} is not a valid date!\nValid dates should be in the format of %Y-%m-%d %H:%M:%S.",
                                    args[i + 1]
                                );
                                final_datetime = datetime;
                                i += 1;
                            }
                            "-p" => picture = Some(args[i + 1].clone()),
                            "-s" => sound = Some(args[i + 1].clone()),
                            "--desc" => {
                                description = Some(args.as_slice()[i + 1..].join(" "));
                                i = args.len();
                            }
                            _ => {
                                bail!("{}: no such option", args[i]);
                            }
                        }
                        i += 2;
                    }

                    if let Some(act_datetime) = final_datetime {
                        return Ok(Command::Remind(Reminder::Once(
                            act_datetime,
                            Media { picture, sound },
                            description,
                        )));
                    }

                    bail!(
                        "No date and time provided to notification! notify once needs the -w field to be set"
                    );
                }

                "daily" => {
                    let mut days: [bool; 7] = [true; 7];
                    while i < args.len() {
                        ensure!(
                            i + 1 != args.len(),
                            "{} was not provided with a parameter!",
                            args[i]
                        );
                        match args[i].as_str() {
                            "-w" => {
                                let time = parse_time(&args, i + 1);
                                ensure!(
                                    time != None,
                                    "{} is not a valid time! Valid time should be in the format of %H:%M:%S.",
                                    args[i + 1]
                                );
                                final_time = time;
                            }
                            "-d" => {
                                let basic_format = Regex::new(r"^[1-7]{1,7}$").unwrap();
                                let compact_format = Regex::new(r"^[1-7]-[1-7]$").unwrap();
                                days = [false; 7];

                                if basic_format.is_match(args[i + 1].as_str()) {
                                    for c in args[i + 1].chars() {
                                        days[c as usize - '1' as usize] = true;
                                    }
                                } else if compact_format.is_match(args[i + 1].as_str()) {
                                    let arg = args[i + 1].as_bytes();
                                    let left = cmp::min(arg[0] as usize, arg[2] as usize);
                                    let right = cmp::max(arg[0] as usize, arg[2] as usize);
                                    for j in left..right {
                                        days[j] = true;
                                    }
                                } else {
                                    days = [true; 7];
                                    warn!(
                                        "{} in not a valid time format. Valid time formats are <n>-<m> and <n1><n2>.. where <_> are numbers from 1-7, notification set to all dates.",
                                        args[i + 1]
                                    );
                                }
                            }
                            "-p" => picture = Some(args[i + 1].clone()),
                            "-s" => sound = Some(args[i + 1].clone()),
                            "--desc" => {
                                description = Some(args.as_slice()[i + 1..].join(" "));
                                i = args.len();
                            }
                            _ => {
                                bail!("{}: no such option", args[i + 1]);
                            }
                        }
                        i += 2;
                    }

                    if let Some(act_time) = final_time {
                        let last_day = Local::now().naive_local().date() - TimeDelta::days(1);
                        return Ok(Command::Remind(Reminder::Daily(
                            act_time,
                            days,
                            last_day,
                            Media { picture, sound },
                            description,
                        )));
                    }
                    bail!(
                        "No date and time provided to notification! notify daily needs the -w field to be set"
                    );
                }

                "interval" => {
                    let mut interval_time: Duration = Duration::zero();
                    while i < args.len() {
                        ensure!(
                            i + 1 != args.len(),
                            "{} was not provided with a parameter!",
                            args[i]
                        );
                        match args[i].as_str() {
                            "-w" => {
                                let datetime = parse_datetime(&args, i + 1);
                                ensure!(
                                    datetime != None,
                                    "{} is not a valid date! Valid dates should be in the format of %Y-%m-%d %H:%M:%S.",
                                    args[i + 1]
                                );
                                final_datetime = datetime;
                                i += 1;
                            }
                            "-i" => {
                                let time_format =
                                    Regex::new(r"^([1-9][0-9]{0,8}[dhmsDHMS]){1,4}$").unwrap();

                                ensure!(
                                    time_format.is_match(args[i + 1].as_str()),
                                    "{} is not a valid time identifier! Valid time identifiers should be in the format of ([1-9][0-9]{{0,8}}[dhmsDHMS]){{1,4}}.",
                                    args[i + 1]
                                );
                                let time_indicator = Regex::new(r"[dhmsDHMS]").unwrap();
                                let mut indicator_indexes =
                                    time_indicator.find_iter(args[i + 1].as_str());
                                let mut prev = 0;
                                let mut next: usize;
                                loop {
                                    let next_res = indicator_indexes.next();
                                    if next_res != None {
                                        next = next_res.unwrap().start();
                                        let time_type = &args[i + 1][next..next + 1];
                                        let time = args[i + 1][prev..next].parse::<i64>().unwrap();
                                        interval_time += match time_type {
                                            "d" => Duration::days(time),
                                            "h" => Duration::hours(time),
                                            "m" => Duration::minutes(time),
                                            "s" => Duration::seconds(time),
                                            _ => {
                                                bail!(
                                                    "{} is not a valid time type! Valid time types are: d, h, m, s",
                                                    time_type
                                                );
                                            }
                                        };
                                        prev = next + 1;
                                    } else {
                                        break;
                                    }
                                }
                            }
                            "-p" => picture = Some(args[i + 1].clone()),
                            "-s" => sound = Some(args[i + 1].clone()),
                            "--desc" => {
                                description = Some(args.as_slice()[i + 1..].join(" "));
                                i = args.len();
                            }
                            _ => {
                                bail!("{}: no such option", args[i + 1]);
                            }
                        }
                        i += 2;
                    }

                    if let Some(act_datetime) = final_datetime {
                        return Ok(Command::Remind(Reminder::SpecificInterval(
                            act_datetime,
                            interval_time,
                            Media { picture, sound },
                            description,
                        )));
                    }
                    bail!(
                        "No date and time provided to notification! notify daily needs the -w field to be set"
                    );
                }
                "custom" => Ok(Command::NoCommand),
                _ => {
                    bail!("{}: no such type of notification!", args[2]);
                }
            }
        }
        "help" | "-h" => Ok(Command::Help),
        "add" => Ok(Command::NoCommand),
        "show" => Ok(Command::NoCommand),
        "note" => {
            ensure!(args.len() >= 3, "Note command needs something to note!");

            let mut description: Option<String> = None;
            let mut name: Option<String> = None;
            let mut i: usize = 2;

            while i < args.len() {
                ensure!(
                    args.len() != i + 1,
                    "{} was not provided with a parameter!",
                    args[i]
                );
                match args[i].as_str() {
                    "-n" => name = Some(args[i + 1].clone()),
                    "--desc" => {
                        description = Some(args.as_slice()[i + 1..].join(" "));
                        i = args.len();
                    }
                    _ => {
                        bail!("{}: no such option", args[i]);
                    }
                }
                i += 2;
            }

            if let (Some(act_description), Some(act_name)) = (description, name) {
                return Ok(Command::Note(Note {
                    name: act_name,
                    text: act_description,
                }));
            }

            bail!(
                "No name or descriptions provided to note! note needs the -n and the --desc fields to be set"
            );
        }
        "delete" => Ok(Command::NoCommand),
        _ => {
            bail!(
                "{} is not a valid argument! Valid arguments are start, stop, remind, note, help, show, add, delete.",
                args[1]
            );
        }
    }
}

#[test]
fn test_empty() {
    let v: Vec<String> = vec![];
    assert!(parse_options(v).is_err());
}

#[test]
fn test_start() {
    let Ok(res) = parse_options(vec![String::new(), String::from("start")]) else {
        panic!("error");
    };
    match res {
        Command::Start => {}
        _ => {
            panic!("wrong command returned");
        }
    }
}

#[test]
fn test_stop() {
    let Ok(res) = parse_options(vec![String::new(), String::from("stop")]) else {
        panic!("error");
    };
    match res {
        Command::Stop => {}
        _ => {
            panic!("wrong command returned");
        }
    }
}

#[test]
fn test_help() {
    let Ok(res) = parse_options(vec![String::new(), String::from("help")]) else {
        panic!("error");
    };
    match res {
        Command::Help => {}
        _ => {
            panic!("wrong command returned");
        }
    }
}

#[test]
fn test_notify() {
    let sound = String::from("s");
    let picture = String::from("p");
    let media = Media {
        sound: Some(sound.clone()),
        picture: Some(picture.clone()),
    };
    let datetime_str = "2020-11-27 5:12:4";
    let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
        .expect("datetime conversion failed");
    let datetime_string = String::from(datetime_str);
    let time_str = "3:15:3";
    let time = NaiveTime::parse_from_str(time_str, "%H:%M:%S").expect("time conversion failed");
    let time_string = String::from(time_str);
    let description1 = String::from("usefull description");
    let description2 = String::from("useless description");
    let notify = String::from("notify");
    let opt_w = String::from("-w");
    let opt_s = String::from("-s");
    let opt_p = String::from("-p");
    let opt_desc = String::from("--desc");

    #[allow(unused_assignments)]
    let mut res = Command::NoCommand;
    match parse_options(vec![
        String::new(),
        notify.clone(),
        String::from("once"),
        opt_w.clone(),
        String::from(&datetime_string[0..10]),
        String::from(&datetime_string[11..]),
        opt_s.clone(),
        sound.clone(),
        opt_p.clone(),
        picture.clone(),
        opt_desc.clone(),
        description1.clone(),
        description2.clone(),
    ]) {
        Ok(command) => {
            res = command;
        }
        Err(err) => {
            panic!("{err}");
        }
    };
    let Command::Remind(Reminder::Once(datetime_res, media_res, description_res)) = res else {
        panic!("wrong command returned");
    };
    if datetime_res != datetime
        || media_res != media
        || description_res.unwrap() != description1.clone() + " " + &description2
    {
        panic!("once test failed");
    }

    let Ok(res) = parse_options(vec![
        String::new(),
        notify.clone(),
        String::from("daily"),
        opt_w.clone(),
        time_string.clone(),
        String::from("-d"),
        String::from("25"),
        opt_s.clone(),
        sound.clone(),
        opt_p.clone(),
        picture.clone(),
        opt_desc.clone(),
        description1.clone(),
    ]) else {
        panic!("error");
    };

    let Command::Remind(Reminder::Daily(time_res, days, _last_day, media_res, description_res)) =
        res
    else {
        panic!("wrong command returned");
    };
    if time_res != time
        || media_res != media
        || description_res.unwrap() != description1
        || days != [false, true, false, false, true, false, false]
    {
        panic!("daily test failed");
    }

    let Ok(res) = parse_options(vec![
        String::new(),
        notify.clone(),
        String::from("interval"),
        opt_w.clone(),
        String::from(&datetime_string[0..10]),
        String::from(&datetime_string[11..]),
        String::from("-i"),
        String::from("12h2m1s"),
        opt_s.clone(),
        sound.clone(),
        opt_p.clone(),
        picture.clone(),
        opt_desc.clone(),
        description1.clone(),
    ]) else {
        panic!("error");
    };
    let Command::Remind(Reminder::SpecificInterval(
        datetime_res,
        duration_res,
        media_res,
        description_res,
    )) = res
    else {
        panic!("wrong command returned");
    };
    if datetime_res != datetime
        || duration_res != TimeDelta::seconds(1 + 60 * 2 + 12 * 3600)
        || media_res != media
        || description_res.unwrap() != description1
    {
        panic!("interval test failed");
    }
}

#[test]
fn test_note() {
    let Ok(res) = parse_options(vec![
        String::new(),
        String::from("note"),
        String::from("-n"),
        String::from("Jhon"),
        String::from("--desc"),
        String::from("cool"),
        String::from("not"),
    ]) else {
        panic!("error");
    };
    let Command::Note(note) = res else {
        panic!("wrong command returned");
    };
    if note.name != String::from("Jhon") || note.text != String::from("cool not") {
        panic!("note test failed");
    }
}

// TODO: put date format into configuration file
fn parse_datetime(args: &Vec<String>, index: usize) -> Option<NaiveDateTime> {
    if args.len() + 1 < index {
        return None;
    }

    let arg = &(args[index].clone() + &args[index + 1]);
    let mut date_res = NaiveDateTime::parse_from_str(arg, "%Y-%m-%d %H:%M:%S");

    if let Ok(date) = date_res {
        return Some(date);
    } else {
        date_res = NaiveDateTime::parse_from_str(
            format!("{} 00:00:00", arg).as_str(),
            "%Y-%m-%d %H:%M:%S",
        );
        if let Ok(date) = date_res {
            return Some(date);
        } else {
            None
        }
    }
}

fn parse_time(args: &Vec<String>, index: usize) -> Option<NaiveTime> {
    if args.len() < index {
        return None;
    }

    let arg = &args[index];
    let mut time_res = NaiveTime::parse_from_str(arg, "%H:%M:%S");

    if let Ok(time) = time_res {
        return Some(time);
    } else {
        time_res =
            NaiveTime::parse_from_str(format!("{} 00:00:00", arg).as_str(), "%Y-%m-%d %H:%M:%S");
        if let Ok(time) = time_res {
            return Some(time);
        } else {
            None
        }
    }
}
