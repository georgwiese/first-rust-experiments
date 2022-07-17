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
            Ok(Config { filename, query })
        } else {
            Err("Unexpected number of arguments!")
        }
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
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
