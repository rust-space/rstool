#[macro_use]
extern crate clap;

use clap::App;
use rstool::{command, logo};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    command::run(matches).unwrap_or_else(|_err| {
        logo::output();
    });
}
