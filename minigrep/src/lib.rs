use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 3 {
            let filename = args[1].clone();
            let query = args[2].clone();
            Ok(Config{filename, query})
        } else {
            Err("Unexpected number of arguments!")
        }
    }
}

pub fn run (config: &Config) -> Result<(), Box<dyn Error>>{
    println!("{:?}", config);
    let contents = fs::read_to_string(&config.filename)?;
    println!();
    println!("Content of file:");
    println!("{}", contents);
    Ok(())
}