use std::error::Error;  //allows for some better errors
use std::fs;            //the library that will allow us to parse files
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
}




pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    //println!("With text:\n{}", contents);
    for line in search(&config.query, &contents).iter() {
        println!("{}",line);
    }

    Ok(())
}


pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 { //make sure enough arguments are given
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })    }
}

/*
 * Currently, our test is failing because we always return an empty vector. 
 * To fix that and implement search, our program needs to follow these steps:
    Iterate through each line of the contents.
    Check whether the line contains our query string.
    If it does, add it to the list of values we’re returning.
    If it doesn’t, do nothing.
    Return the list of results that match.
*/
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