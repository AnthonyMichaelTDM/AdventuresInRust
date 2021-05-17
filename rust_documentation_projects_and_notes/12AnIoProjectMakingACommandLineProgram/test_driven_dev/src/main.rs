use std::env;           //the library that will allow us to do stuff and stuff
use std::process;       //allows for some better error handling

mod lib;
use crate::lib::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); //read the argument values, and collect them into a string vector

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
        
    });
    /*
     * Using unwrap_or_else allows us to define some custom, non-panic! error handling. 
     * If the Result is an Ok value, this method’s behavior is similar to unwrap: it returns the inner value Ok is wrapping. 
     * However, if the value is an Err value, this method calls the code in the closure, 
     * which is an anonymous function we define and pass as an argument to unwrap_or_else.
    */

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    //handling errors in run with an if let
    if let Err(e) = lib::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    } /*
     * We use if let rather than unwrap_or_else to check whether run returns an Err value and call process::exit(1) if it does. 
     * The run function doesn’t return a value that we want to unwrap in the same way that Config::new returns the Config instance.
     * Because run returns () in the success case, we only care about detecting an error,
     * so we don’t need unwrap_or_else to return the unwrapped value because it would only be ().
    */
}
