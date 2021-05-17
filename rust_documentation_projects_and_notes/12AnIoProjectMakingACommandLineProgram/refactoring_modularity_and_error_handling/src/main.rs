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
 * 
 * 
*/

use std::env;           //the library that will allow us to do stuff and stuff
use std::process;       //allows for some better error handling

mod lib;
use crate::lib::Config;

/*
 * currently, there are 4 major problems with this project:
 * First, our main function now performs two tasks: it parses arguments and reads files. For such a small function, 
   this isn’t a major problem. However, if we continue to grow our program inside main, 
   the number of separate tasks the main function handles will increase. As a function gains responsibilities, 
   it becomes more difficult to reason about, harder to test, and harder to change without breaking one of its parts. 
   It’s best to separate functionality so each function is responsible for one task.

 * This issue also ties into the second problem: although query and filename are configuration variables to our program, 
   variables like contents are used to perform the program’s logic. The longer main becomes, 
   the more variables we’ll need to bring into scope; the more variables we have in scope, 
   the harder it will be to keep track of the purpose of each. 
   It’s best to group the configuration variables into one structure to make their purpose clear.

 * The third problem is that we’ve used expect to print an error message when reading the file fails, 
   but the error message just prints Something went wrong reading the file. 
   Reading a file can fail in a number of ways: for example, the file could be missing, 
   or we might not have permission to open it. Right now, regardless of the situation, 
   we’d print the Something went wrong reading the file error message, which wouldn’t give the user any information!

 * Fourth, we use expect repeatedly to handle different errors, and if the user runs our program without specifying enough arguments, 
   they’ll get an index out of bounds error from Rust that doesn’t clearly explain the problem. 
   It would be best if all the error-handling code were in one place so future maintainers had only one place to consult 
   in the code if the error-handling logic needed to change. Having all the error-handling code in one place will also ensure 
   that we’re printing messages that will be meaningful to our end users.
 * 
 * there are quite a few things we need to do to solve these issues:
     * Extracting the Argument Parser
     * Grouping Configuration Values
         * Creating a Constructor for Config
     * Fixing the Error Handling
         * Improving the Error Message
         * Returning a Result from new Instead of Calling panic!
         * Calling Config::new and Handling Errors
     * Extracting Logic from main
         * Returning Errors from the run Function
         * Handling Errors Returned from run in main
     * Splitting Code into a Library Crate
*/

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

//At the moment, we’re returning a tuple, but then we immediately break that tuple into individual parts again. 
//This is a sign that perhaps we don’t have the right abstraction yet.
// fn parse_config(args: &[String]) -> (&str, &str) {
//     //save the second and third arguments, ignoring the 1st because it's just the path to the binary
//     let query = &args[1];
//     let filename = &args[2];
//     (query, filename)
// }
//replaced by the methods of the struct
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     /* The Trade-Offs of Using clone
//      * There’s a tendency among many Rustaceans to avoid using clone to fix ownership problems because of its runtime cost. 
//      * In Chapter 13, you’ll learn how to use more efficient methods in this type of situation. 
//      * But for now, it’s okay to copy a few strings to continue making progress because you’ll make these copies only once 
//      * and your filename and query string are very small. 
//     */
//     Config { query, filename }
// }