use std::env;


fn main() {
    /////////////////////////////////
    // Reading the Argument Values //
    /////////////////////////////////
    /*
     * To enable minigrep to read the values of command line arguments we pass to it, 
     * we’ll need a function provided in Rust’s standard library, which is std::env::args. 
     * 
     * This function returns an iterator of the command line arguments that were given to minigrep. 
     * 
     * We’ll cover iterators fully in Chapter 13. 
     * For now, you only need to know two details about iterators: 
     *  iterators produce a series of values, 
     *  and we can call the collect method on an iterator to turn it into a collection, such as a vector, 
     *  containing all the elements the iterator produces.
     * 
     ** Note on The args Function and Invalid Unicode **
     * Note that std::env::args will panic if any argument contains invalid Unicode. 
     * If your program needs to accept arguments containing invalid Unicode, use std::env::args_os instead. 
     * That function returns an iterator that produces OsString values instead of String values. 
     * We’ve chosen to use std::env::args here for simplicity, 
     * because OsString values differ per platform and are more complex to work with than String values.
     * 
     * we call env::args, and we immediately use collect to turn the iterator into a vector containing all the values produced by the iterator.
     * We can use the collect function to create many kinds of collections, 
     * so we explicitly annotate the type of args to specify that we want a vector of strings. 
     * Although we very rarely need to annotate types in Rust, 
     * collect is one function you do often need to annotate because Rust isn’t able to infer the kind of collection you want.
    */
    /*
    Notice that when running this program, the first value in the vector is "target/debug/minigrep", which is the name of our binary. 
    This matches the behavior of the arguments list in C, letting programs use the name by which they were invoked in their execution. 
    It’s often convenient to have access to the program name in case you want to print it in messages or change behavior of the program 
    based on what command line alias was used to invoke the program. But for the purposes of this chapter, 
    we’ll ignore it and save only the two arguments we need.
    */
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);


    /////////////////////////////////////////////
    // Saving the Argument Values in Variables //
    /////////////////////////////////////////////
    
    
    let args: Vec<String> = env::args().collect(); //read the argument values, and collect them into a string vector
    //save the second and third arguments, ignoring the 1st because it's just the path to the binary
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
}
