use std::env::Args;

pub struct Config {
    pub file: String
}

pub fn get_config(mut args:Args) -> Result<Config, &'static str>{
    if args.len() <= 1 {
        return Err("Should add an argument");
    }
    Ok(Config{
        file: match args.nth_back(0) {
            Some(s) => s,
            None => todo!(),
        }
    })
}