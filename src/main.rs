use colored::{Color, ColoredString, Colorize};
use serde::Deserialize;
use serde_json::from_str;
use std::io::{self, BufRead};

#[derive(Debug, Deserialize)]
struct Log {
    name: String,
    msg: String,
    level: i32,
    time: String,
    target: String,
}

fn get_level(level_num: i32) -> ColoredString {
    match level_num {
        10 => "TRACE".to_owned().yellow(),
        20 => "DEBUG".to_owned().bright_magenta(),
        30 => "INFO".to_owned().green(),
        40 => "WARN".to_owned().yellow(),
        50 => "ERROR".to_owned().red(),
        60 => "FATAL".to_owned().bright_red(),
        _ => "UNKNOWN".to_owned().white(),
    }
}

fn main() {
    // For testing: Read the log from file

    // let lines_in_string = std::fs::read_to_string("error.log").expect("Can't read 'error.log'");
    // let lines = lines_in_string.lines().collect::<Vec<&str>>();

    let stdin = io::stdin();

    let primary_color = Color::TrueColor {
        r: 202,
        g: 188,
        b: 155,
    };

    for line in stdin.lock().lines() {
        if line.is_err() {
            continue;
        };

        match from_str::<Log>(&line.unwrap()) {
            Ok(log) => {
                let time = format!("[{}]", log.time);
                let level = get_level(log.level);
                let target = format!("on {}:", log.target);
                let name = format!(": {}", log.name);

                let formatted_log = format!(
                    "{} {}{} {} {}",
                    time.dimmed(),
                    level,
                    name.color(primary_color),
                    target.color(primary_color),
                    log.msg.green()
                );

                println!("{}", formatted_log);
            }
            Err(_) => continue,
        }
    }

    std::process::exit(0);
}
