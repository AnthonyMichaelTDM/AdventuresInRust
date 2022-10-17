use std::error::Error;  //allows for some better errors
use std::fs;            //the library that will allow us to parse files
use std::env;           //gives access to environment stuff

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /* REMOVING A CLONE USING AN ITERATOR:
    previously, this function looked like this:
    ```
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {return Err("not enough arguments");}
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })    
    }
    ```

    At the time, we said not to worry about the inefficient `clone` calls because we would
    remove them in the future. Well, that time is now!

    Previously, we needed a `clone` here because we have a slice with `String` elements in the parameter `args`, 
    but the `build` function doesn't own `args`. 
    To return ownership of a `Config` instance, 
    we had to clone the values from the `query` and `filename` fields of `Config` so the `Config` instance can own its values.

    With our new knowledge about iterators, 
    we can change the `build` function to take ownership of an iterator as its argument instead of borrowing a slice. 
    We’ll use the iterator functionality instead of the code that checks the length of the slice and indexes into specific locations. 
    This will clarify what the `Config::build` function is doing because the iterator will access the values.

    Once `Config::build` takes ownership of the iterator and stops using indexing operations that borrow, 
    we can move the `String` values from the iterator into `Config` rather than calling clone and making a new allocation.
    
    continues in main.rs
    */
    /*
     * USING THE RETURNED ITERATOR DIRECTLY
     * continued from main.rs
     * 
     * we've updated the signature of the `Config::build` function so the parameter `args` has a generic type with the trait bounds 'impl Iterator<Item = String>` instead of `&[String]`.
     * This usage of the `impl Trait` syntax is discussed in ch10, 
     * and means that `args` can be any type that implements the `Iterator` type and returns `String` items.
     * 
     * Because we're taking ownership of `args` and we'll be mutating `args` by iterating over it, we add the `mut` keyword into the specification of the `args` parameter to make it mutable.
     */
    pub fn build(
        mut args: impl Iterator<Item = String>, // define the parameter args as a mutable variable of a type that implements the `Iterator` trait and returns `String` values. 
    ) -> Result<Config, &'static str> {
        /* USING ITERATOR TRAIT METHODS INSTEAD OF INDEXING
        Next, we'll fix the body of 'Config::build'.
        Because 'args' implements the `Iterator` trait, we know we can call the `next` method on it.
        */
        args.next(); //skip first arg ( path to executable)
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        //use the var function from the env module to get an environment variable named IGNORE_CASE, 
        //is_ok makes it boolean (false if there's an error (environment variable not set), true if ok (environment variable set))
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })    
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    //choose the search method based on environment variables
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    println!(" ");
    for line in results {
        println!("{}",line);
    }

    Ok(())
}

/* MAKING CODE CLEANER WITH ITERATOR ADAPTORS
we can also take advantage of iterators to make the `search` function, 
which used to be:
```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
```
cleanser and more concise
*/
pub fn search<'a> (query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines() // get a iterator from contents where every item is a line in contents
        .filter(|line| line.contains(query)) // filter out lines that don't contain the query
        .collect() // collect the resulting iterator into a vector, which is then implicitly returned because there is no semicolon
}

/*
we can do the same thing here by modifying the closure in the filter adaptor

for reference, this function used to be: 
```
pub fn search_case_insensitive<'a> (query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
```
*/
pub fn search_case_insensitive<'a> (query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines() // get a iterator from contents where every item is a line in contents
        .filter(|line| line.to_lowercase().contains(&query)) // filter out lines that don't contain the query, even when case is ignored
        .collect() // collect the resulting iterator into a vector, which is then implicitly returned because there is no semicolon
}


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