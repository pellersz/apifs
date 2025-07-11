use regex::Regex;
use chrono::{NaiveDateTime, NaiveTime};
use std::cmp;

pub fn parse_options(args: Vec<String>) -> Option<Command> {
    if args.len() < 2 {
        println!("Too few arguments");
        return None;
    }

    // from the command line arguments return the command 
    match args[1].as_str() {
        "start"     =>  Some(Command::Start),
        "stop"      =>  Some(Command::Stop),
        "notify"    =>  { 
            if args.len() < 3 {
                println!("Notify command needs a type!");
                return None;
            }

            let mut final_datetime: Option<NaiveDateTime> = None;
            let mut final_time: Option<NaiveTime> = None;
            let mut picture: Option<String> = None;
            let mut sound: Option<String> = None;
            let mut description: Option<String> = None;
            let mut i: usize = 3;

            match args[2].as_str() {
                "once"      => {
                    while i < args.len() {
                        if i + 1 == args.len() {
                            println!("{} was not provided with a parameter!", args[i]);
                            return None;
                        }
                        match args[i].as_str() {
                            "-w"        => {
                                let datetime = parse_datetime(&args, i + 1);
                                if let Some(_) = datetime {
                                    final_datetime = datetime;
                                } else {
                                    println!("{} is not a valid date!\nValid dates should be in the format of %Y-%m-%d %H:%M:%S.", args[i + 1]);
                                    return None;
                                }
                            }, 
                            "-p"        => picture = Some(args[i + 1].clone()),
                            "-s"        => sound = Some(args[i + 1].clone()),
                            "--desc"    => description = Some(args[i + 1].clone()),
                            _           => {
                                println!("{}: no such option", args[i + 1]);
                                return None;
                            }
                        }
                        i += 2;                          
                    }

                    if let Some(act_datetime) = final_datetime {
                        return Some(Command::Remind(Reminder::Once(act_datetime, Media { picture, sound }, description)));
                    }
                    
                    println!("No date and time provided to notification! notify once needs the -w field to be set");
                    None
                },
         
                "daily"     => {
                    let mut days: [bool; 7] = [true; 7];
                    while i < args.len() {
                        if i + 1 == args.len() {
                            println!("{} was not provided with a parameter!", args[i]);
                            return None;
                        }
                        match args[i].as_str() {
                            "-w"        => {
                                let time = parse_time(&args, i + 1);
                                if let Some(_) = time {
                                    final_time = time;
                                } else {
                                    println!("{} is not a valid date!\nValid dates should be in the format of %Y-%m-%d %H:%M:%S.", args[i + 1]);
                                    return None;
                                }
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
                                    println!("{} in not a valid time format. Valid time formats are <n>-<m> and <n1><n2>.. where <_> are numbers from 1-7", args[i + 1]);
                                }
                            }
                            "-p"        => picture = Some(args[i + 1].clone()),
                            "-s"        => sound = Some(args[i + 1].clone()),
                            "--desc"    => description = Some(args[i + 1].clone()),
                            _           => {
                                println!("{}: no such option", args[i + 1]);
                                return None;
                            }
                        }
                        i += 2;                          
                    }

                    if let Some(act_time) = final_time {
                        return Some(Command::Remind(Reminder::Daily(act_time, days, Media { picture, sound }, description)));
                    }
                    println!("No date and time provided to notification! notify daily needs the -w field to be set");
                    None
                },

                "interval"  => {
                    // TODO: change this to a Duration
                    let mut interval_time: u32 = 0;
                    while i < args.len() {
                        if i + 1 == args.len() {
                            println!("{} was not provided with a parameter!", args[i]);
                            return None;
                        }
                        match args[i].as_str() {
                            "-w"        => {
                                let datetime = parse_datetime(&args, i + 1);
                                if let Some(_) = datetime {
                                    final_datetime = datetime;
                                } else {
                                    println!("{} is not a valid date!\nValid dates should be in the format of %Y-%m-%d %H:%M:%S.", args[i + 1]);
                                    return None;
                                }
                            }, 
                            "-i"        => {
                                let time_format = Regex::new(r"^([1-9][0-9]{0,8}[dhmsDHMS]){1,4}$").unwrap();
                                    
                                if time_format.is_match(args[i + 1].as_str()) {
                                    let time_indicator = Regex::new(r"[dhmsDHMS]").unwrap();
                                    let mut indicator_indexes = time_indicator.find_iter(args[i + 1].as_str());
                                    let mut prev = indicator_indexes.next().unwrap().start();
                                    let mut next: usize;
                                    loop {
                                        let next_res = indicator_indexes.next();
                                        if next_res != None { 
                                            next = next_res.unwrap().start();
                                            let time_type = &args[i + 1][prev..prev+1];
                                            let time = args[i + 1][prev..next].parse::<usize>().unwrap() as u32;
                                            interval_time += time * match time_type {
                                                "d" => 86400,
                                                "h" => 3600,
                                                "m" => 60,
                                                "s" => 1,
                                                _   => {
                                                    println!("{} is not a valid time type! Valid time types are: d, h, m, s", time_type);
                                                    return None;
                                                }
                                            };
                                            prev = next;
                                        } else {
                                            break;
                                        }
                                    }
                                } else {
                                    println!("{} is not a valid time identifier! Valid time identifiers should be in the format of %Y-%m-%d %H:%M:%S.", args[i + 1]);
                                    return None;
                                }
                            },
                            "-p"        => picture = Some(args[i + 1].clone()),
                            "-s"        => sound = Some(args[i + 1].clone()),
                            "--desc"    => description = Some(args[i + 1].clone()),
                            _           => {
                                println!("{}: no such option", args[i + 1]);
                                return None;
                            }
                        }
                        i += 2;                          
                    }

                    if let Some(act_datetime) = final_datetime {
                        return Some(Command::Remind(Reminder::SpecificInterval(act_datetime, interval_time, Media { picture, sound }, description)));
                    }
                    println!("No date and time provided to notification! notify daily needs the -w field to be set");
                    None
                },
                "custom"    => None,
                _           => {
                    println!("{}: no such type of notification!", args[2]);
                    None
                }
            }
        },
        "help"      =>  Some(Command::Help),
        "add"       =>  None,
        "show"      =>  None,
        "note"      =>  None,
        "delete"    =>  None,
        _           =>  {
            println!("{} is not a valid argument! Valid arguments are start, stop, remind, note, help, show, add, delete.", args[1]);
            return None;
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
}

pub enum Reminder {
    Once(NaiveDateTime, Media, Option<String>),
    MoreThanOnce(Vec<NaiveDateTime>, Media, Option<String>),
    Daily(NaiveTime, [bool; 7], Media, Option<String>),
    SpecificInterval(NaiveDateTime, u32, Media, Option<String>),
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

