use std::{fs::File, io::Read};

pub mod config;

pub fn get_string_from_file(file: &mut File) -> Result<String, &'static str> {
    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Ok(_) => {
            Ok(file_content)
        },
        Err(_) => return Err("Unknown format"),
    }
}

pub trait LineFiltering {
    fn filter(&mut self);
    fn get_filtered_string(&mut self) -> String;
    fn new(content_to_filter: String) -> Self;
}

pub struct FileString {
    pub lines: Vec<String>
}

impl LineFiltering for FileString {
    fn filter(&mut self) {
        self.lines.dedup()
    }

    fn get_filtered_string(&mut self) -> String {
        let lines_count = self.lines.len();
        for i in 0..lines_count {
            self.lines.get_mut(i).unwrap().push('\n');
        }
        let lines_iter = self.lines.iter();
        String::from_iter(lines_iter.map(|x| x.as_str())).trim().to_string()
    }

    fn new(content_to_filter: String) -> Self {
        FileString { lines: content_to_filter.lines().map(|x| String::from(x)).collect() }
    }
}