use anyhow::{bail, ensure, Error};
use regex::Regex;
use chrono::{Duration, NaiveDateTime, NaiveTime};
use std::{cmp, format};

pub fn parse_options(args: Vec<String>) -> Result<Command, Error> {
    ensure!(args.len() >= 2, "Too few arguments");

    // from the command line arguments return the command 
    match args[1].as_str() {
        "start"     =>  Ok(Command::Start),
        "stop"      =>  Ok(Command::Stop),
        "notify"    =>  { 
            ensure!(args.len() >= 3, "Notify command needs a type!");

            let mut final_datetime: Option<NaiveDateTime> = None;
            let mut final_time: Option<NaiveTime> = None;
            let mut picture: Option<String> = None;
            let mut sound: Option<String> = None;
            let mut description: Option<String> = None;
            let mut i: usize = 3;

            match args[2].as_str() {
                "once"      => {
                    while i < args.len() {
                        ensure!(args.len() != i + 1, "{} was not provided with a parameter!", args[i]);
                        match args[i].as_str() {
                            "-w"        => {
                                let datetime = parse_datetime(&args, i + 1);
                                ensure!(datetime != None, "{} is not a valid date!\nValid dates should be in the format of %Y-%m-%d %H:%M:%S.", args[i + 1]);
                                final_datetime = datetime;
                            }, 
                            "-p"        => picture = Some(args[i + 1].clone()),
                            "-s"        => sound = Some(args[i + 1].clone()),
                            "--desc"    => description = Some(args[i + 1].clone()),
                            _           => {
                                bail!("{}: no such option", args[i + 1]);
                            }
                        }
                        i += 2;                          
                    }

                    if let Some(act_datetime) = final_datetime {
                        return Ok(Command::Remind(Reminder::Once(act_datetime, Media { picture, sound }, description)));
                    }
                    
                    bail!("No date and time provided to notification! notify once needs the -w field to be set");
                },
         
                "daily"     => {
                    let mut days: [bool; 7] = [true; 7];
                    while i < args.len() {
                        ensure!(i + 1 != args.len(), "{} was not provided with a parameter!", args[i]);
                        match args[i].as_str() {
                            "-w"        => {
                                let time = parse_time(&args, i + 1);
                                ensure!(time != None, "{} is not a valid date! Valid dates should be in the format of %Y-%m-%d %H:%M:%S.", args[i + 1]);
                                final_time = time;
                            }, 
                            "-d"        => {
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
                                    days = [true;7];
                                    warn!("{} in not a valid time format. Valid time formats are <n>-<m> and <n1><n2>.. where <_> are numbers from 1-7, notification set to all dates.", args[i + 1]);
                                }
                            }
                            "-p"        => picture = Some(args[i + 1].clone()),
                            "-s"        => sound = Some(args[i + 1].clone()),
                            "--desc"    => description = Some(args[i + 1].clone()),
                            _           => {
                                bail!("{}: no such option", args[i + 1]);
                            }
                        }
                        i += 2;                          
                    }

                    if let Some(act_time) = final_time {
                        return Ok(Command::Remind(Reminder::Daily(act_time, days, Media { picture, sound }, description)));
                    }
                    bail!("No date and time provided to notification! notify daily needs the -w field to be set");
                },

                "interval"  => {
                    let mut interval_time: Duration = Duration::zero();
                    while i < args.len() {
                        ensure!(i + 1 != args.len(), "{} was not provided with a parameter!", args[i]);
                        match args[i].as_str() {
                            "-w"        => {
                                let datetime = parse_datetime(&args, i + 1);
                                ensure!(datetime != None, "{} is not a valid date! Valid dates should be in the format of %Y-%m-%d %H:%M:%S.", args[i + 1]);
                                final_datetime = datetime;
                            }, 
                            "-i"        => {
                                let time_format = Regex::new(r"^([1-9][0-9]{0,8}[dhmsDHMS]){1,4}$").unwrap();
                                
                                ensure!(time_format.is_match(args[i + 1].as_str()), "{} is not a valid time identifier! Valid time identifiers should be in the format of %Y-%m-%d %H:%M:%S.", args[i + 1]);
                                let time_indicator = Regex::new(r"[dhmsDHMS]").unwrap();
                                let mut indicator_indexes = time_indicator.find_iter(args[i + 1].as_str());
                                let mut prev = indicator_indexes.next().unwrap().start();
                                let mut next: usize;
                                loop {
                                    let next_res = indicator_indexes.next();
                                    if next_res != None { 
                                        next = next_res.unwrap().start();
                                        let time_type = &args[i + 1][prev..prev+1];
                                        let time = args[i + 1][prev..next].parse::<usize>().unwrap() as i64;
                                        interval_time += match time_type {
                                            "d" => Duration::days(time),
                                            "h" => Duration::hours(time),
                                            "m" => Duration::minutes(time),
                                            "s" => Duration::seconds(time),
                                            _   => {
                                                bail!("{} is not a valid time type! Valid time types are: d, h, m, s", time_type);
                                            }
                                        };
                                        prev = next;
                                    } else {
                                        break;
                                    }
                                }
                            },
                            "-p"        => picture = Some(args[i + 1].clone()),
                            "-s"        => sound = Some(args[i + 1].clone()),
                            "--desc"    => description = Some(args[i + 1].clone()),
                            _           => {
                                bail!("{}: no such option", args[i + 1]);
                            }
                        }
                        i += 2;                          
                    }

                    if let Some(act_datetime) = final_datetime {
                        return Ok(Command::Remind(Reminder::SpecificInterval(act_datetime, interval_time, Media { picture, sound }, description)));
                    }
                    bail!("No date and time provided to notification! notify daily needs the -w field to be set");
                },
                "custom"    => Ok(Command::NoCommand),
                _           => {
                    bail!("{}: no such type of notification!", args[2]);
                }
            }
        },
        "help"      =>  Ok(Command::Help),
        "add"       =>  Ok(Command::Help),
        "show"      =>  Ok(Command::Help),
        "note"      =>  Ok(Command::Help),
        "delete"    =>  Ok(Command::Help),
        _           =>  {
            bail!("{} is not a valid argument! Valid arguments are start, stop, remind, note, help, show, add, delete.", args[1]);
        }
    }  
}

// TODO: put date format into configuration file
fn parse_datetime(args: &Vec<String>, index: usize) -> Option<NaiveDateTime> {
    if args.len() < index {
        return None
    }

    let arg = &args[index];
    let mut date_res = NaiveDateTime::parse_from_str(arg, "%Y-%m-%d %H:%M:%S");
    
    if let Ok(date) = date_res {
        return Some(date); 
    } else {
        date_res = NaiveDateTime::parse_from_str(format!("{} 00:00:00", arg).as_str(), "%Y-%m-%d %H:%M:%S");
        if let Ok(date) = date_res { 
            return Some(date);
        } else {
            None
        }
    }
}

fn parse_time(args: &Vec<String>, index: usize) -> Option<NaiveTime> {
    if args.len() < index {
        return None
    }

    let arg = &args[index];
    let mut time_res = NaiveTime::parse_from_str(arg, "%H:%M:%S");
    
    if let Ok(time) = time_res {
        return Some(time); 
    } else {
        time_res = NaiveTime::parse_from_str(format!("{} 00:00:00", arg).as_str(), "%Y-%m-%d %H:%M:%S");
        if let Ok(time) = time_res { 
            return Some(time);
        } else {
            None
        }
    }
}


pub enum Command {
    Remind(Reminder),
    Note(Note),
    Start,
    Stop,
    Help,
    AddMedia(String, String),
    Delete(String),
    Show(Data),
    NoCommand
}

pub enum Reminder {
    Once(NaiveDateTime, Media, Option<String>),
    MoreThanOnce(Vec<NaiveDateTime>, Media, Option<String>),
    Daily(NaiveTime, [bool; 7], Media, Option<String>),
    SpecificInterval(NaiveDateTime, Duration, Media, Option<String>),
}

pub struct Media {
    picture: Option<String>,
    sound: Option<String>,
}

pub struct Note {
    name: String,
    text: String,
    media: Media,
}

pub enum Data {
    Reminder(String),
    Note(String),
    Picture(String),
    Sound(String),
}

