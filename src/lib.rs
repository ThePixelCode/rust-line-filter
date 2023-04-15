use std::{fs::{File, OpenOptions}, io::{Read, Write, stdin, Seek, SeekFrom::Start}, process::exit};

use config::Config;
use string_filter::StringFilter;
use traits::LineFiltering;

pub mod config;
pub mod traits;
pub mod string_filter;

struct Files {
    pub input: Option<File>,
    pub output: Option<File>
}

fn set_files_from_config(config: &Config) -> Result<Files, &'static str> {
    let input_file = match &config.file {
        Some(path) => {
            let mut new_open_option = OpenOptions::new();
            let mut open_option = new_open_option.read(true);
            if config.output.is_none() {
                open_option = open_option.write(true);
            }
            match open_option.open(path) {
                Ok(file) => Some(file),
                Err(_) => return Err("Error trying to open file"),
            }
        },
        None => None,
    };
    let output_file = match &config.output {
        Some(path) => {
            let file = match OpenOptions::new().write(true).create(true).open(path) {
                Ok(file) => file,
                Err(_) => return Err("Error trying to open file"),
            };
            Some(file)
        },
        None => None,
    };
    Ok(Files{
        input: input_file,
        output: output_file
    })
}

fn filter_string(string: String, config: &Config) -> String {
    let mut filter = StringFilter::new(string);
    filter.filter(&config.order);
    filter.get_filtered_string()
}

fn print_help_and_exit() {
    println!("Exec format:
    [command] -> Reads from standard input and prints to standard output
    [command] -h -> Prints this help and exit
    [command] -O -> Orders the text before filtering
    [command] -f [file] -> Reads from [file]
    [command] -i -> Reads from standard input (Default)
    [command] -o [file] -> Outputs to [file], if -o is not set and -f is set then outputs to [file] set in -f,
                           in other case outputs to standard out");
    exit(0);
    
}

fn get_string_from_file(file: &mut File) -> Result<String, &'static str> {
    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Ok(_) => Ok(file_content),
        Err(_) => return Err("Unknown format"),
    }
}

fn save_string_to_file(string_to_save: String, file: &mut File) -> Result<(), &'static str> {
    match file.seek(Start(0)) {
        Ok(_) => (),
        Err(_) => return Err("Error during seek"),
    }
    match file.set_len(0) {
        Ok(_) => (),
        Err(_) => return Err("File was not opened as writable"),
    }
    match file.write_all(string_to_save.as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err("Got an unexpected error"),
    }
    match file.sync_all() {
        Ok(_) => (),
        Err(_) => return Err("An Error happened when synchronizing memory with filesystem"),
    }
    Ok(())
}

fn get_string_from_stdin() -> Result<String, &'static str> {
    let mut buffer = Vec::new();
    match stdin().lock().read_to_end(&mut buffer) {
        Ok(_) => {
            match String::from_utf8(buffer) {
                Ok(string) => Ok(string),
                Err(_) => return Err("Invalid characters found"),
            }
        },
        Err(_) => return Err("Something went wrong when reading from standard input"),
    }
}

fn get_string_to_filter(config: &Config, files: &mut Files) -> Result<String, &'static str> {
    if config.stdin {
        match get_string_from_stdin() {
            Ok(string) => return Ok(string),
            Err(error) => return Err(error),
        };
    } else {
        match files.input {
            Some(ref mut file) => {
                match get_string_from_file(file) {
                    Ok(string) => return Ok(string),
                    Err(error) => return Err(error),
                }
            },
            None => return Err("Something went really really wrong"),
        };
    }
}

pub fn process(config: Config) -> Result<(), &'static str> {
    if config.help {
        print_help_and_exit();
    }
    let mut files = match set_files_from_config(&config) {
        Ok(files) => files,
        Err(error) => return Err(error),
    };
    let string_to_filter = match get_string_to_filter(&config, &mut files) {
        Ok(string) => string,
        Err(error) => return Err(error),
    };
    let filtered_string = filter_string(string_to_filter, &config);
    match config.output {
        Some(_) => {
            return save_string_to_file(filtered_string, &mut files.output.unwrap())
        },
        None => {
            if config.stdin {
                println!("{}", filtered_string);
                Ok(())
            } else {
                return save_string_to_file(filtered_string, &mut files.input.unwrap());
            }
        },
    }
}
