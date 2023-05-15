use clap::ArgMatches;

use crate::libs::{time, ip};

pub fn time_handler(matches: &ArgMatches) {
    let std= matches.get_one::<String>("std").map(|s| s.as_str()).unwrap();
    if matches.contains_id("convert") {
        if matches.contains_id("datetime") {
            let datetime = matches.get_one::<String>("datetime").map(|s| s.as_str()).unwrap();
            println!("{}", time::str_to_timestamp(datetime));
        } else {
            let timestamp = matches.get_one::<String>("timestamp").map(|s| s.as_str()).unwrap();
            let ts = time::timestamp_to_string(timestamp, std)
                .expect("error: unable to parse <timestamp> as a i64");
            println!("{}", ts);
        }
    } else {
        println!("{}", time::get_now_string(std));
    }
}

pub fn ip_handler(matches: &ArgMatches) {
    if matches.contains_id("resolve") {
        let domain = matches.get_one::<String>("resolve").map(|s| s.as_str()).unwrap();
        let ip = ip::resolve(domain).unwrap().unwrap();
        println!("<{}> IP address: {:?}", domain, ip);
    } else if matches.contains_id("public") {
        let ip = ip::get_public_ip().expect("error: get public ip execute failed");
        println!("Public IP address: {}", ip);
    } else {
        let ip = ip::get_local_ip().expect("error: get local ip execute failed");
        println!("Local IP address: {}", ip);
    }
}