use std::error::Error;
use std::fs;
use std::env;
use colored::Colorize;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // Read file

    let file: String = fs::read_to_string(&config.filename)?;

    println!("Searching for {} in {}", config.query, config. filename);
    println!("-----------------------\n");

    // echo it out

    let result = if config.case_sensitive {
        search(&config.query, &file)
    } else {
        search_case_insensitive(&config.query, &file)
    };

    let len_query = &config.query.len();

    for line in result {
        // grab indices where the word appears

        let v: Vec<_> = line.match_indices(&config.query).map(|(i, _)|i).collect();
        let mut last = 0;
        for ind in v {
            print!("{}",&line[last..ind]);
            print!("{}",&line[ind..ind + len_query].bold().red());
            last = ind + len_query;
        }
        print!("{}", &line[last..]);
        println!("");
    }

    Ok(())

}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut res: Vec<&str> = vec![];
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }

    res
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }

    res
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}


impl Config {

    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {

        // path to program
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => {
                return Err("Query not specified \
                               USE: ./greppy \'query\' \'filename\'");
            }
        };

        let filename = match args.next() {
            Some(filename) => filename,
            None => {
                return Err("Filename not specified \
                               USE: ./greppy \'query\' \'filename\'");
            }
        };
        

        if let Some(_) = args.next() {
            return Err("Too many arguments \
                       USE: ./greppy \'query\' \'filename\'");
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {  query, filename , case_sensitive})
    }
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}

