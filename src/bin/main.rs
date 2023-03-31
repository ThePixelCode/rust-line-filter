use rust_line_filter::config::Config;
use std::{env::args, process::exit};

fn main() {
    let config = match Config::new(args()) {
        Ok(config) => config,
        Err(error) => {
            println!("{}", error);
            exit(1);
        },
    };
    match config.process() {
        Ok(_) => (),
        Err(error) => {
            println!("{}", error);
            exit(2);
        },
    }
}
