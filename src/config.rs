use std::env::Args;

pub struct Config {
    pub help: bool,
    pub file: Option<String>,
    pub stdin: bool,
    pub output: Option<String>,
    pub order: bool
}

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
}
