use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    /*
     * Most errors aren’t serious enough to require the program to stop entirely.
     * Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.
     *
     * For example, if you try to open a file and that operation fails because the file doesn’t exist,
     * you might want to create the file instead of terminating the process.
     */
    let f = File::open("hello.txt");
    /*
     * File::open returns a result,
     * Recall from “Handling Potential Failure with the Result Type” in Chapter 2 that the Result enum is defined as having two variants, Ok and Err, as follows:
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
     * T and E are generic type parameters
     * T represents the type of the value that will be returned in a success case within the Ok variant,
     * and E represents the type of the error that will be returned in a failure case within the Err variant.
     *
     * Because Result has these generic type parameters, we can use the Result type and the functions
     * that the standard library has defined on it in many different situations where the
     * successful value and error value we want to return may differ.
     */
    //now, let's handle 'f'
    /*
     * Here we tell Rust that when the result is Ok, return the inner file value out of the Ok variant,
     * and we then assign that file handle value to the variable f.
     * After the match, we can use the file handle for reading or writing.
     */
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // }; //Note that, like the Option enum, the Result enum and its variants have been brought into scope by the prelude, so we don’t need to specify Result:: before the Ok and Err variants in the match arms.

    //Matching on Different Errors
    /*
     * The previous code will panic! no matter why File::open failed.
     * What we want to do instead is take different actions for different failure reasons:
     * if File::open failed because the file doesn’t exist, we want to create the file and return the handle to the new file.
     * If File::open failed for any other reason—for example,
     * because we didn’t have permission to open the file—we still want the code to panic!
     */
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    //a cleaner, more concise, way to do that, which will be covered further in future notes, is as follows
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //shortcuts for panic on error: unwrap and expect
    /*
    Using match works well enough, but it can be a bit verbose and doesn’t always communicate intent well.
    The Result<T, E> type has many helper methods defined on it to do various tasks.
    One of those methods, called unwrap, is a shortcut method that is implemented just like the match expression we wrote first.
    If the Result value is the Ok variant, unwrap will return the value inside the Ok.
    If the Result is the Err variant, unwrap will call the panic! macro for us. Here is an example of unwrap in action:
    */
    let _f = File::open("hello.txt").unwrap();

    /*
    Another method, expect, which is similar to unwrap, lets us also choose the panic! error message.
    Using expect instead of unwrap and providing good error messages can convey your intent
    and make tracking down the source of a panic easier. The syntax of expect looks like this:
    */
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
}

/* Propagating Errors
 *
 * When you’re writing a function whose implementation calls something that might fail,
 * instead of handling the error within this function, you can return the error to the calling code
 * so that it can decide what to do. This is known as propagating the error and gives more control to the calling code,
 * where there might be more information or logic that dictates how the error should be handled
 * than what you have available in the context of your code.
 */
fn _read_username_from_file_verbose_propagation() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), //if opening the file doesn't work, return an error
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),   //if reading a string from the file works, return the string
        Err(e) => Err(e), //if reading a string from the file doesn't work, return an error
    }
} //remember, you'll have to handle the potential errors when calling the funtion








/* A Shortcut for Propagating Errors: the ? Operator */
fn _read_username_from_file_concise_propagation() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; //this ? operator will return the value inside an Ok to the variable f.
                                          //If an error occurs, the ? operator will return early out of the whole 
                                          //function and give any Err value to the calling code. 
                                          //The same thing applies to the ? at the end of the read_to_string call.
    let mut s = String::new();
    f.read_to_string(&mut s)?; //The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values previously
                               // If the value is an Err,
                               //the Err will be returned from the whole function as if we had used the return keyword
                               //so the error value gets propagated to the calling code.

    /*
     * There is a difference between what the match expression from the previos function does and what the ? operator does:
     * error values that have the ? operator called on them go through the from function,
     * defined in the From trait in the standard library, which is used to convert errors from one type into another.
     * When the ? operator calls the from function, the error type received is converted into the error type defined in the return type
     * of the current function. This is useful when a function returns one error type to represent all the ways a function might fail,
     * even if parts might fail for many different reasons.
     * As long as each error type implements the from function to define how to convert itself to the returned error type,
     * the ? operator takes care of the conversion automatically.
     */
    Ok(s)
}






//The ? operator eliminates a lot of boilerplate and makes this function’s implementation simpler. 
//We could even shorten this code further by chaining method calls immediately after the ?, as shown below
fn _read_username_from_file_super_concise_propogation() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}









//rememeber: The ? Operator Can Be Used in Functions That Return Result