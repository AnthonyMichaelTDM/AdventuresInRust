use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); //read the argument values, and collect them into a string vector
    
    //save the second and third arguments, ignoring the 1st because it's just the path to the binary
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    //In main, we’ve added a new statement: fs::read_to_string takes the filename, opens that file, 
    //and returns a Result<String> of the file’s contents.
    
    println!("With text:\n{}", contents);

    /*
     * generally, functions are clearer and easier to maintain if each function is responsible for only one idea. 
     * The other problem is that we’re not handling errors as well as we could. 
     * 
     * The program is still small, so these flaws aren’t a big problem, but as the program grows, it will be harder to fix them cleanly. 
     * 
     * It’s good practice to begin refactoring early on when developing a program, 
     * because it’s much easier to refactor smaller amounts of code. We’ll do that next.
    */
}
