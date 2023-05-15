mod libs;
mod handler;
mod logo;

#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        Some(("time", sub_matches)) => {
            handler::time_handler(sub_matches)
        }
        Some(("ip", sub_matches)) => {
            handler::ip_handler(sub_matches)
        }
        _ => {
            logo::output()
        }
    }
}