use rust_line_filter::config::Config;
use rust_line_filter::process;
use std::{env::args, process::exit};

fn main() {
    let config = match Config::new(args()) {
        Ok(config) => config,
        Err(error) => {
            println!("{}", error);
            exit(1);
        },
    };
    match process(config) {
        Ok(_) => (),
        Err(error) => {
            println!("{}", error);
            exit(2);
        },
    }
}
