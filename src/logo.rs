//! 命令行logo模块

use ansi_term::Color;

pub fn output() {
    let logo = generate();
    println!("{}", logo);
}

fn generate() -> String {
    let logo = r#"
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
    format!(
        "{}{}{}",
        green.paint(logo),
        yellow.paint("    Welcome to use rstool!"),
        blue.paint("\n\n")
    )
}
