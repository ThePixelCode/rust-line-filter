use rust_line_filter::{config::get_config, get_string_from_file, FileString, LineFiltering};
use std::{fs::OpenOptions, io::Write};

fn main() {
    let args = std::env::args();
    let config = match get_config(args){
        Ok(config) => config,
        Err(_) => todo!(),
    };
    let mut file = match OpenOptions::new().read(true).write(true).open(config.file){
        Ok(file) => file,
        Err(error) => panic!("{}", error),
    };
    let file_string = match get_string_from_file(&mut file) {
        Ok(string) => string,
        Err(_) => todo!(),
    };

    let mut file_string = FileString::new(file_string);
    file_string.filter();
    let new_string_to_put_in_file = file_string.get_filtered_string();
    file.write_all(new_string_to_put_in_file.as_bytes()).unwrap();
    file.sync_all().unwrap();
}
