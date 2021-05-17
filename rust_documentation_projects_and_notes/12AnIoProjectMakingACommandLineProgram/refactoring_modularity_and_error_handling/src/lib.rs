use std::error::Error;  //allows for some better errors
use std::fs;            //the library that will allow us to parse files

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



//issue with this is that there isn't much more we can do with teh error handling, the message isn't very helpful
// fn run(config: Config) {
//     let contents = fs::read_to_string(config.filename)
//         .expect("Something went wrong reading the file");
//     //In main, we’ve added a new statement: fs::read_to_string takes the filename, opens that file, 
//     //and returns a Result<String> of the file’s contents.

//     println!("With text:\n{}", contents);
// }
//fixed form
/**We’ve made three significant changes here. 
 * First, we changed the return type of the run function to Result<(), Box<dyn Error>>. 
 * This function previously returned the unit type, (), and we keep that as the value returned in the Ok case.
 * For the error type, we used the trait object Box<dyn Error> 
 * (and we’ve brought std::error::Error into scope with a use statement at the top). 
 * We’ll cover trait objects in Chapter 17. 
 * For now, just know that Box<dyn Error> means the function will return a type that implements the Error trait, 
 * but we don’t have to specify what particular type the return value will be. 
 * This gives us flexibility to return error values that may be of different types in different error cases. 
 * The dyn keyword is short for “dynamic.”
 * 
 * Second, we’ve removed the call to expect in favor of the ? operator, as we talked about in Chapter 9. 
 * Rather than panic! on an error, ? will return the error value from the current function for the caller to handle.
 * 
 * Third, the run function now returns an Ok value in the success case. 
 * We’ve declared the run function’s success type as () in the signature, which means we need to wrap the unit type value in the Ok value. 
 * his Ok(()) syntax might look a bit strange at first, 
 * but using () like this is the idiomatic way to indicate that we’re calling run for its side effects only; 
 * it doesn’t return a value we need.
*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}