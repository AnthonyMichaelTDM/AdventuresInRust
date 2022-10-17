use std::error::Error;  //allows for some better errors
use std::fs;            //the library that will allow us to parse files
use std::env;           //gives access to environment stuff

/*
 * In this section, we’ll add the searching logic to the minigrep program by using the Test-driven development (TDD) process. 
 * This software development technique follows these steps:
     *1 Write a test that fails and run it to make sure it fails for the reason you expect.
     *2 Write or modify just enough code to make the new test pass.
     *3 Refactor the code you just added or changed and make sure the tests continue to pass.
     *4 Repeat from step 1!
 * This process is just one of many ways to write software, but TDD can help drive code design as well. 
 * Writing the test before you write the code that makes the test pass helps to maintain high test coverage throughout the process.
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

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

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 { //make sure enough arguments are given
            return Err("not enough arguments");
        }
        //don't worry about these inefficient .clone() calls, we'll get rid of them in ch13
        let query = args[1].clone();
        let filename = args[2].clone();

        //use the var function from the env module to check for an environment variable named CASE_INSENSITIVE
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        /*
        using the is_err method on the Result to check whether it’s an error and therefore unset, 
        which means it should do a case-sensitive search. If the CASE_INSENSITIVE environment variable is set to anything, 
        is_err will return false and the program will perform a case-insensitive search. 
        We don’t care about the value of the environment variable, just whether it’s set or unset, 
        so we’re checking is_err rather than using unwrap, expect, or any of the other methods we’ve seen on Result
        */

        Ok(Config { query, filename, case_sensitive })    
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    //choose the search method based on environment variables
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    println!(" ");
    for line in results {
        println!("{}",line);
    }

    Ok(())
}

pub fn search<'a> (query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    //iterate through each line
    for line in contents.lines() { // Rust has a helpful method to handle line-by-line iteration of strings, conveniently named lines
        // check whetehr the line contains our query string
        if line.contains(query) {
            //add it to the list of values we're returning
            results.push(line);
        } else {
            //do nothing
        }
    }
    //return the list of results that match
    results
}

pub fn search_case_insensitive<'a> (query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();
    //iterate through each line
    for line in contents.lines() { // Rust has a helpful method to handle line-by-line iteration of strings, conveniently named lines
        // check whetehr the line contains our query string
        if line.to_lowercase().contains(&query) {
            //add it to the list of values we're returning
            results.push(line);
        } else {
            //do nothing
        }
    }
    //return the list of results that match
    results
}