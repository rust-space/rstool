pub mod time;

use ansi_term::Color;
use clap::{App, Arg, SubCommand};

use crate::time::TimeTool;

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
        let tt = TimeTool::new();
        let std = matches.value_of("std").unwrap();

        if matches.is_present("convert") {
            if matches.is_present("datetime") {
                let datetime = matches.value_of("datetime").unwrap();
                println!("{}", tt.str2timestamp(datetime));
            } else {
                let timestamp = matches.value_of("timestamp").unwrap_or_default();
                let ts = tt
                    .timestamp2string(timestamp, std)
                    .expect("error: unable to parse <timestamp> as a i64");
                println!("{}", ts);
            }
        } else {
            println!("{}", tt.now2string(std));
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
