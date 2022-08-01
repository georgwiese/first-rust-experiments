use std::env::Args;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();
        let filename = match args.next() {
            None => return Err("Didn't get a file name"),
            Some(arg) => arg,
        };
        let query = match args.next() {
            None => return Err("Didn't get a query"),
            Some(arg) => arg,
        };

        Ok(Config { filename, query })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::search;

    #[test]
    fn one_result() {
        let query = "Foo";
        let contents = "\
fo foo fooo
Bar Foo Foo
foo bar foo";
        assert_eq!(search(query, contents), vec!["Bar Foo Foo"]);
    }
}
