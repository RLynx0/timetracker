#![allow(unused)] // TODO: Remove this when more things are implemented

use std::{
    fs,
    io::{self, Write, stdin, stdout},
    path::{Path, PathBuf},
    process::exit,
    str::FromStr,
};

use clap::Parser;
use rev_lines::RawRevLines;

use crate::{config::Config, entry::ActivityEntry, opt::Opt};

mod config;
mod entry;
mod files;
mod format_string;
mod opt;

fn main() {
    let opt = Opt::parse();

    let config = match load_or_create_config(opt.config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load or create config: {e}");
            exit(1)
        }
    };

    println!("{config:?}")

    // let config_result = toml::from_str::<Config>(&config_str);
    // println!("{config_result:#?}");
}

fn load_or_create_config(custom_path: Option<PathBuf>) -> anyhow::Result<Config> {
    let config_path = match custom_path {
        None => files::default_config_path()?,
        Some(p) => p,
    };
    if fs::exists(&config_path)? {
        let config_str = fs::read_to_string(config_path)?;
        Ok(toml::from_str(&config_str)?)
    } else {
        println!("I couldn't find the required config at {config_path:?}");
        println!("Let me guide you through creating your configuration!");
        let config = make_guided_config()?;
        let config_str = toml::to_string(&config)?;
        if let Some(p) = config_path.parent() {
            fs::create_dir_all(p)?;
        }
        fs::write(&config_path, config_str)?;
        println!("Saved generated configuration to {config_path:?}");
        println!("\n--------\n");
        Ok(config)
    }
}

fn make_guided_config() -> anyhow::Result<Config> {
    let default = toml::from_str::<Config>(include_str!("../default_config.toml"))
        .expect("Default config must be valid");

    println!("\nFirst, please enter your name. (Firstname Lastname)");
    print!("Your Name: ");
    stdout().flush()?;
    let mut employee_name = String::new();
    stdin().read_line(&mut employee_name)?;

    println!("\nWe'll need your personnel number as well.");
    print!("Your emplyee id: ");
    stdout().flush()?;
    let mut employee_number = String::new();
    stdin().read_line(&mut employee_number)?;

    println!("\nPlease also enter your cost center ID.");
    print!("Your cost center: ");
    stdout().flush()?;
    let mut cost_center = String::new();
    stdin().read_line(&mut cost_center)?;

    print!("Your performance type: ");
    stdout().flush()?;
    let mut performance_type = String::new();
    stdin().read_line(&mut performance_type)?;

    print!("Your accounting cycle: ");
    stdout().flush()?;
    let mut accounting_cycle = String::new();
    stdin().read_line(&mut accounting_cycle)?;

    println!("\nOkay, that's it!");

    let default_attendance = &default.default_attendance;
    println!("\nI have set your default attendance type to {default_attendance}.",);
    println!("This is probably what you want, but in case it isn't -");
    println!("You can always manually edit the generated config file.");
    println!("There are a few more options there that were also set automatically!");

    Ok(Config {
        employee_name,
        employee_number,
        cost_center,
        performance_type,
        accounting_cycle,
        ..default
    })
}

fn get_last_state_entry(path: &Path) -> anyhow::Result<Option<ActivityEntry>> {
    let file = fs::File::open(path)?;
    let mut rev_lines = RawRevLines::new(file);
    match rev_lines.next() {
        Some(res) => {
            let entry = &String::from_utf8(res?)?;
            Ok(Some(ActivityEntry::from_str(entry)?))
        }
        None => Ok(None),
    }
}
