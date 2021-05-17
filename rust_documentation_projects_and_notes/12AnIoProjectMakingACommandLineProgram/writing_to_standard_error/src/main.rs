/* Separation of Concerns for Binary Projects
 * The organizational problem of allocating responsibility for multiple tasks to the main function is common to many binary projects. 
 * As a result, the Rust community has developed a process to use as a guideline for splitting the separate concerns of a binary program 
 * when main starts getting large. The process has the following steps:
    
    Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
    As long as your command line parsing logic is small, it can remain in main.rs.
    When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

 * The responsibilities that remain in the main function after this process should be limited to the following:

    Calling the command line parsing logic with the argument values
    Setting up any other configuration
    Calling a run function in lib.rs
    Handling the error if run returns an error

 * This pattern is about separating concerns: main.rs handles running the program, and lib.rs handles all the logic of the task at hand. 
 * Because you can’t test the main function directly, this structure lets you test all of your program’s logic by moving it into functions in lib.rs. 
 * The only code that remains in main.rs will be small enough to verify its correctness by reading it.
*/

use std::env;           //the library that will allow us to do stuff and stuff
use std::process;       //allows for some better error handling

mod lib;
use crate::lib::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); //read the argument values, and collect them into a string vector

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); //use the eprintln! macro to output to standard error
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
        eprintln!("Application error: {}", e); //use the eprintln! macro to output to standard error

        process::exit(1);
    } /*
     * We use if let rather than unwrap_or_else to check whether run returns an Err value and call process::exit(1) if it does. 
     * The run function doesn’t return a value that we want to unwrap in the same way that Config::new returns the Config instance.
     * Because run returns () in the success case, we only care about detecting an error,
     * so we don’t need unwrap_or_else to return the unwrapped value because it would only be ().
    */
}
/* Summary
 * This chapter recapped some of the major concepts you’ve learned so far and covered how to perform common I/O operations in Rust. 
 * By using command line arguments, files, environment variables, and the eprintln! macro for printing errors, 
 * you’re now prepared to write command line applications. By using the concepts in previous chapters, 
 * your code will be well organized, store data effectively in the appropriate data structures, handle errors nicely, and be well tested.
 * 
 * Next, we’ll explore some Rust features that were influenced by functional languages: closures and iterators. 
*/