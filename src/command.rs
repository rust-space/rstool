//! 命令执行模块

use crate::libs::{ip, time};
use clap::ArgMatches;

#[derive(Debug)]
pub struct CommandErr {
    pub message: String,
}

impl std::error::Error  for CommandErr {}

impl std::fmt::Display for CommandErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

trait Handler {
    fn handle(&self);
}

struct TimeHandler<'a> {
    pub matches: &'a ArgMatches,
}

struct IpHandler<'a> {
    pub matches: &'a ArgMatches,
}

impl<'a> Handler for TimeHandler<'a> {
    fn handle(&self) {
        let std = self
            .matches
            .get_one::<String>("std")
            .map(|s| s.as_str())
            .unwrap();
        if self.matches.contains_id("convert") {
            if self.matches.contains_id("datetime") {
                let datetime = self
                    .matches
                    .get_one::<String>("datetime")
                    .map(|s| s.as_str())
                    .unwrap();
                println!("{}", time::str_to_timestamp(datetime));
            } else {
                let timestamp = self
                    .matches
                    .get_one::<String>("timestamp")
                    .map(|s| s.as_str())
                    .unwrap();
                let ts = time::timestamp_to_string(timestamp, std)
                    .expect("error: unable to parse <timestamp> as a i64");
                println!("{}", ts);
            }
        } else {
            println!("{}", time::get_now_string(std));
        }
    }
}

impl<'a> Handler for IpHandler<'a> {
    fn handle(&self) {
        if self.matches.contains_id("resolve") {
            let domain = self
                .matches
                .get_one::<String>("resolve")
                .map(|s| s.as_str())
                .unwrap();
            let ip = ip::resolve(domain)
                .expect("error: resolve domain to IP addr failed")
                .unwrap();
            println!("<{}> IP address: {:?}", domain, ip);
        } else if self.matches.contains_id("public") {
            let ip = ip::get_public_ip().expect("error: get public IP addr failed");
            println!("public IP address: {}", ip);
        } else {
            let ip = ip::get_local_ip().expect("error: get local IP addr failed");
            println!("local IP address: {}", ip);
        }
    }
}

pub fn run(matches: ArgMatches) -> Result<(), CommandErr> {
    let handler: Box<dyn Handler> = match matches.subcommand() {
        Some(("time", sub_matches)) => Box::new(TimeHandler {
            matches: sub_matches,
        }),
        Some(("ip", sub_matches)) => Box::new(IpHandler {
            matches: sub_matches,
        }),
        _ => {
            return Err(CommandErr {
                message: "unknown command".to_string(),
            });
        }
    };
    handler.handle();
    Ok(())
}
