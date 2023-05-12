use ansi_term::Color;
use chrono::{Local, NaiveDateTime, TimeZone, Utc};
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("rstool")
        .version("0.0.1")
        .author("Jin Peng <554085731@qq.com>")
        .about("A development toolset developed in Rust")
        .subcommand(
            SubCommand::with_name("time")
                .about("Time tools")
                .arg(
                    Arg::new("now")
                        .short('n')
                        .long("now")
                        .help("Output current time")
                        .conflicts_with("convert"),
                )
                .arg(
                    Arg::new("convert")
                        .short('c')
                        .long("convert")
                        .help("Convert time format")
                        .conflicts_with("now"),
                )
                .arg(
                    Arg::new("timestamp")
                        .short('t')
                        .long("timestamp")
                        .takes_value(true)
                        .help("Convert timestamp to datetime string")
                        .conflicts_with("datetime"),
                )
                .arg(
                    Arg::new("datetime")
                        .short('d')
                        .long("datetime")
                        .takes_value(true)
                        .help("Convert datetime string to timestamp")
                        .conflicts_with("timestamp"),
                )
                .arg(
                    Arg::new("std")
                        .short('s')
                        .long("standard")
                        .takes_value(true)
                        .possible_values(&["rfc2822", "rfc3339", "timestamp", ""])
                        .default_value(""),
                ),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("time") {
        let std = matches.value_of("std").unwrap();

        if matches.is_present("convert") {
            if matches.is_present("datetime") {
                let input = matches.value_of("datetime").unwrap();
                let datetime = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S").unwrap();
                println!("{}", datetime.timestamp())
            } else {
                let input = matches.value_of("timestamp").unwrap_or_default();
                let timestamp = if input.is_empty() {
                    Utc::now().timestamp()
                } else {
                    input.parse::<i64>().unwrap()
                };
                let datetime = Utc.timestamp_opt(timestamp, 0).unwrap();
                match std {
                    "timestamp" => println!("{}", datetime.timestamp()),
                    "rfc2822" => println!("{}", datetime.to_rfc2822()),
                    "rfc3339" => println!("{}", datetime.to_rfc3339()),
                    _ => println!("{}", datetime.format("%Y-%m-%d %H:%M:%S").to_string()),
                }
            }
        } else {
            let now = Local::now();
            match std {
                "timestamp" => println!("{}", now.timestamp()),
                "rfc2822" => println!("{}", now.to_rfc2822()),
                "rfc3339" => println!("{}", now.to_rfc3339()),
                _ => println!("{}", now.format("%Y-%m-%d %H:%M:%S").to_string()),
            }
        }
    } else {
        let linux_logo = r#"
          _              _ 
         | |            | |
 _ __ ___| |_ ___   ___ | |
| '__/ __| __/ _ \ / _ \| |
| |  \__ \ || (_) | (_) | |
|_|  |___/\__\___/ \___/|_|                          
    "#;
        let green = Color::Green.bold();
        let yellow = Color::Yellow.bold();
        let blue = Color::Blue.bold();
        let ansi_logo = format!(
            "{}{}{}",
            green.paint(linux_logo),
            yellow.paint("    Welcome to use rstool!"),
            blue.paint("\n\n")
        );
        println!("{}", ansi_logo);
    }
}
