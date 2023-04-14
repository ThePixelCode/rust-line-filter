use std::{env::Args, fs::{File, OpenOptions}, io::{Read, Write, stdin, Seek, SeekFrom::Start}, process::exit};

use crate::{string_filter::StringFilter, traits::LineFiltering};

pub struct Config {
    help: bool,
    file: Option<String>,
    stdin: bool,
    output: Option<String>,
    order: bool
}

/*
Exec format:
[command] -> Reads from standard input and prints to standard output
[command] -h -> Prints help and exit
[command] -f [file] -> Reads from [file]
[command] -i -> Reads from standard input (Default)
[command] -o [file] -> Outputs to [file], if -o is not set and -f is set then should output to [file] set in -f,
                       in other case should output to standard out
*/
impl Config {
    pub fn new(mut args:Args) -> Result<Self, &'static str> {
        let mut help = false;
        let mut file: Option<String> = None;
        let mut stdin = false;
        let mut output: Option<String> = None;
        let mut order = false;
        loop {
            let arg = match args.next() {
                Some(arg) => arg,
                None => break,
            };
            if arg == "-h" {
                help = true;
                continue
            }
            if arg == "-i" {
                stdin = true;
                continue
            }
            if arg == "-f" {
                let file_arg = match args.next() {
                    Some(file) => file,
                    None => return Err("File not set")
                };
                file = Some(file_arg);
                continue
            }
            if arg == "-o" {
                let file_arg = match args.next() {
                    Some(file) => file,
                    None => return Err("File not set")
                };
                output = Some(file_arg);
                continue
            }
            if arg == "-O" {
                order = true;
            }
        }
        if file.is_some() && stdin {
            return Err("Incompatible arguments found");
        }
        if file.is_none() && !stdin {
            stdin = true;
        }
        Ok(Config {
            file,
            help,
            stdin,
            output,
            order
        })
    }

    fn get_string_from_file(file: &mut File) -> Result<String, &'static str> {
        let mut file_content = String::new();
        match file.read_to_string(&mut file_content) {
            Ok(_) => {
                Ok(file_content)
            },
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

    pub fn process(self) -> Result<(), &'static str>{
        if self.help {
            Self::print_help_and_exit();
        }
        if self.stdin {
            let string_to_filter = match Self::get_string_from_stdin() {
                Ok(string) => string,
                Err(error) => return Err(error),
            };
            let mut filter = StringFilter::new(string_to_filter);
            filter.filter(self.order);
            let filtered_string = filter.get_filtered_string();
            match self.output {
                Some(path) => {
                    let mut file = match OpenOptions::new().write(true).create(true).open(path) {
                        Ok(file) => file,
                        Err(_) => return Err("Error trying to open file"),
                    };
                    match Self::save_string_to_file(filtered_string, &mut file) {
                        Ok(_) => (),
                        Err(error) => return Err(error),
                    }
                },
                None => println!("{}", filtered_string),
            }
            return Ok(())
        }
        match self.file {
            Some(path) => {
                match self.output {
                    Some(output_path) => {
                        let mut file = match OpenOptions::new().read(true).open(path) {
                            Ok(file) => file,
                            Err(_) => return Err("Error trying to open file"),
                        };
                        let mut output_file = match OpenOptions::new().write(true).create(true).open(output_path) {
                            Ok(file) => file,
                            Err(_) => return Err("Error trying to open file"),
                        };
                        let string_to_filter = match Self::get_string_from_file(&mut file) {
                            Ok(string) => string,
                            Err(error) => return Err(error),
                        };
                        let mut filter = StringFilter::new(string_to_filter);
                        filter.filter(self.order);
                        let filtered_string = filter.get_filtered_string();
                        match Self::save_string_to_file(filtered_string, &mut output_file) {
                            Ok(_) => (),
                            Err(error) => return Err(error),
                        }
                    },
                    None => {
                        let mut file = match OpenOptions::new().read(true).write(true).open(path) {
                            Ok(file) => file,
                            Err(_) => return Err("Error trying to open file"),
                        };
                        let string_to_filter = match Self::get_string_from_file(&mut file) {
                            Ok(string) => string,
                            Err(error) => return Err(error),
                        };
                        let mut filter = StringFilter::new(string_to_filter);
                        filter.filter(self.order);
                        let filtered_string = filter.get_filtered_string();
                        match Self::save_string_to_file(filtered_string, &mut file) {
                            Ok(_) => (),
                            Err(error) => return Err(error),
                        }
                    },
                }
            },
            None => return Err("Something went really really wrong..."),
        }
        Ok(())
    }
}
