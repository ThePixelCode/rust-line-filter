use std::{fs::File, io::Read};

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

