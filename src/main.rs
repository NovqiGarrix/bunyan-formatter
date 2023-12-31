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
    #[serde(rename(deserialize = "otel.name"))]
    http_target: Option<String>,
    #[serde(rename(deserialize = "http.host"))]
    http_host: Option<String>,
    #[serde(rename(deserialize = "otel.status_code"))]
    http_status_code: Option<String>,
    #[serde(rename(deserialize = "elapsed_milliseconds"))]
    http_took: Option<i32>,
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

fn format_option_field(value: Option<String>, field: &str, primary_color: &Color) -> String {
    if value.is_some() {
        format!(
            "{}{}",
            format_args!(
                "{}",
                format_args!("{}: ", field)
                    .to_string()
                    .color(primary_color.to_owned())
            ),
            value.unwrap().green()
        )
    } else {
        "".to_owned()
    }
}

fn display_each(line: &str, primary_color: Color) {
    if let Ok(log) = from_str::<Log>(line) {
        let time = format!("[{}]", log.time).dimmed();
        let level = get_level(log.level);
        let target = format!(
            "TARGET: {}",
            // format_args!("{}", "TARGET:".dimmed()),
            format_args!("{}", log.target)
        )
        .dimmed();

        let msg = format!(
            "{} {}",
            format_args!("{}", "MSG:").to_string().color(primary_color),
            format_args!("{}", log.msg.green())
        );

        let name = format!(": {}", log.name).color(primary_color);

        let http_host = format_option_field(log.http_host, "HOST", &primary_color);
        let http_target = format_option_field(log.http_target, "ROUTE", &primary_color);
        let http_status_code =
            format_option_field(log.http_status_code, "STATUS CODE", &primary_color);
        let http_took = format_option_field(
            if log.http_took.is_some() {
                Some(format!("{} ms", log.http_took.unwrap()))
            } else {
                None
            },
            "TOOK",
            &primary_color,
        );

        let formatted_log = format!(
            "{} {}{} {} {} {} {} {} {}",
            time, level, name, target, msg, http_host, http_target, http_status_code, http_took
        );

        println!("{}", formatted_log);
    }
}

fn lines_from_file() -> Vec<String> {
    let lines_in_string = std::fs::read_to_string("error.log").expect("Can't read 'error.log'");
    lines_in_string
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<String>>()
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let empty_string = &String::from("");
    let mode = args.get(1).unwrap_or(empty_string);

    let primary_color = Color::TrueColor {
        r: 202,
        g: 188,
        b: 155,
    };

    if mode == "-d" {
        let lines = lines_from_file();
        for line in lines {
            display_each(&line, primary_color);
        }
    } else {
        for line in io::stdin().lock().lines() {
            if line.is_err() {
                continue;
            }

            display_each(&line.unwrap(), primary_color)
        }
    };
}
