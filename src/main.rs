use clap::Parser;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use input::args::Args;
use input::config::Config;
use run::run;

mod audio;
mod input;
mod instance;
mod properties;
mod render;
mod run;

fn main() -> Result<(), &'static str> {
    let config = match confy::load("vocal", "config") {
        Ok(config) => config,
        Err(_) => {
            let config = Config::default();
            match confy::store("vocal", "config", config.clone()) {
                Ok(_) => config,
                Err(_) => {
                    return Err("Failed to load config");
                }
            }
        }
    };

    match enable_raw_mode() {
        Ok(_) => {}
        Err(_) => {
            return Err("Failed to enable raw keyboard mod");
        }
    }

    match run(config, Args::parse()) {
        Ok(_) => {}
        Err(err) => {
            return Err(err);
        }
    }

    match disable_raw_mode() {
        Ok(_) => {}
        Err(_) => {
            return Err("Failed to disable raw keyboard mod");
        }
    }

    Ok(())
}
