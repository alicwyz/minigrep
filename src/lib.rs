
use std::error::Error; //Error type
use std::fs; //handle files
use std::env;

//$Env:IGNORE_CASE=1;
//Remove-Item Env:IGNORE_CASE

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn new(query: String, file_path: String, ignore_case: bool) -> Config {
        Config { query , file_path, ignore_case }
    }

    pub fn build(
        mut args: impl Iterator<Item = String>, //Generic type with trait bounds
    ) -> Result<Config, &'static str>  {

        args.next(); //Name of the program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string")
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No file path")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config::new(query, file_path, ignore_case))
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { //dynamic type

    let contents = fs::read_to_string(config.file_path)?; //Sends error to caller

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(()) //Ok with void return
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { 
    //Lifetime of return connected to contents

    let result: Vec<&str> = 
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect();
        
    result
    /* 
    let mut results = Vec::new(); //Accumulator

    for line in contents.lines() { //Loop inside iterator
        if line.contains(query) { //Filter
            results.push(line); //Push to accumulator
        }
    }
    results
    */    
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {

    let query = query.to_lowercase();

    let result: Vec<&str> =
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query))
            .collect();
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape."; //"D" dont match

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitve() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}