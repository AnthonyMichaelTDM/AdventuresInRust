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
    
    //use stuff in lib.rs to parse the arguments
    if let Err(e) = lib::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
