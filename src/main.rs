#![warn(clippy::pedantic)]

extern crate clap;

use clap::{Arg, App, crate_version};

mod renamers;

struct Config {
    path: String,
    start: u32
}

impl Config {
    fn new(path: &str, start: &str) -> Self {
        Self {
            path: String::from(path),
            start: start.parse::<u32>().unwrap()
        }
    }
}

fn start_is_valid(val: String) -> Result<(), String> {
    #![allow(clippy::needless_pass_by_value)]
    match val.parse::<u32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Must be a positive integer."))
    }
}

fn main() {
    let cli_matches = App::new("Rust Renamrs")
                        .version(crate_version!())
                        .arg(Arg::with_name("PATH")
                            .help("Path of the directory whose files will be renamed")
                            .required(true)
                            .index(1)
                        )
                        .arg(Arg::with_name("start")
                            .help("Number from which the sequence will start")
                            .short("-s")
                            .long("--start")
                            .default_value("1")
                            .value_name("N")
                            .validator(start_is_valid)
                        )
                        .get_matches();
                        
    let conf = Config::new(cli_matches.value_of("PATH").unwrap(), cli_matches.value_of("start").unwrap());
    renamers::rename_from_folder_name(&conf.path, conf.start).unwrap_or_else(|e| eprintln!("{}", e));
}
