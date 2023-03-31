use std::{fs::File, io::{Read, Write}};

pub mod config;
pub mod traits;
pub mod string_filter;

pub fn get_string_from_file(file: &mut File) -> Result<String, &'static str> {
    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Ok(_) => {
            Ok(file_content)
        },
        Err(_) => return Err("Unknown format"),
    }
}

pub fn save_string_to_file(string_to_save: String, file: &mut File) -> Result<(), &'static str> {
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
