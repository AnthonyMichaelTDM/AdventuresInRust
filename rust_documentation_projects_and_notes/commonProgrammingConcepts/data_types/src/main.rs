use std::io;
fn main() {
    //in rust, there are 2 type subset, scalar and compound
    /*Keep in mind that Rust is a statically typed language, 
    which means that it must know the types of all variables at compile time. 
    The compiler can usually infer what type we want to use based on the value and how we use it.*/
    
    //////scalar types//////
    
    //// integer types ////
    /*integers can be any whole numer within the size range of the specific integer declaration 
    signed integers can be negative or positive, unsigned can only be positive
    when integers overflow, they wrap around*/
    /* declarations
    integers can be of many different sizes, and either signed or unsigned
    signed | unsigned | size
    i8     | u8       | 8 bit integer
    i16    | u16      | 16-bit integer
    i32    | u32      | 32-bit integer  (fastest) (i32 is default)
    i64    | u64      | 64-bit integer
    i128   | u128     | 128-bit ineger
    isize  | usize    | depends on computer architecture (64 bits for 64-bit machines, 32 bits for 32-bit machines, etc.)
    */
    /* integer literals
    You can write integer literals in any of the forms below. 
      Note that all number literals except the byte literal allow a type suffix, 
      such as 57u8, and _ as a visual separator, such as 1_000.
    number literals | example
    decimal         | 100_000
    hex             | 0xff
    octal           | 0o77
    binary          | 0b1111_0000
    byte(u8 only)   | b'A'
    */
    let decimal_int: i32 = 100_000;     // 100000
    let hex_int: i32 = 0xff;            // 255
    let octal_int: i32 = 0o77;          // 63
    let binary_int: i32 = 0b1111_0000;  // 240
    let /*mut*/ byte_int: u8 = b'A';        // 65
    println!("the decimal int: {}", decimal_int);
    println!("the hex int: {}", hex_int);
    println!("the octal int: {}", octal_int);
    println!("the binary int: {}", binary_int);
    println!("the byte int: {}", byte_int);
    //example of integer overflow
    //byte_int += 200;
    //println!("the overflowed int: {}", byte_int);// must compile with the --release flag or it'll panic, but yeah

    //// floating point types ////
    /* there are 2 types of floats, f64 and f32, which are 64-bit and 32-bit respectively
    floats can have decimal places
    */
    let float_64 = 2.0; //defaults to f64
    let float_32: f32 = 3.0; //f32
    println!("{} {}",float_64, float_32);
    /*Floating-point numbers are represented according to the IEEE-754 standard. 
    The f32 type is a single-precision float, and f64 has double precision.*/

    //// numeric operations can be done with integers and floats ////
    // addition
    let sum = 5 + 10;
    println!("(sum) 5 + 10 = {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("(difference) 95.5 - 4.3 = {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("(multiplication) 4 * 30 = {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("(division) 56.7 / 32.2 = {}", quotient);
    // remainder
    let remainder = 43 % 5;
    println!("(modulo) 43 % 5 = {}", remainder);

    //// boolean ////
    //like most languages, booleans are either true or false
    let t = true; //can be declared implicitly
    let f: bool = false; //or explicitly
    println!("{}",t);
    println!("{}",f);
    //The main way to use Boolean values is through conditionals, such as an if expression.

    //// char ////
    //Note that char literals are specified with single quotes, as opposed to string literals, which use double quotes.
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}",c);
    println!("{}",z);
    println!("{}",heart_eyed_cat);
    /*Rust’s char type is four bytes in size and represents a Unicode Scalar Value, 
    which means it can represent a lot more than just ASCII. 
    Accented letters; Chinese, Japanese, and Korean characters; 
    emoji; and zero-width spaces are all valid char values in Rust. 
    Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. 
    However, a “character” isn’t really a concept in Unicode, 
    so your human intuition for what a “character” is may not match up with what a char is in Rust. */



    ////// Compound types //////
    //Rust has 2 types of compound variable types, tuples and arrays

    ////the tuple type ////
    /*A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
    Tuples have a fixed length: once declared, they cannot grow or shrink in size.*/
    /*We create a tuple by writing a comma-separated list of values inside parentheses. 
    Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.*/
    // We’ve added optional type annotations in this example:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
    let (x,y,z) = tup;
    println!("{} _ {} _ {}", x,y,z);
    /*
    This program first creates a tuple and binds it to the variable tup. 
    It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z. 
    This is called destructuring, because it breaks the single tuple into three parts.
    */
    /*
    In addition to destructuring through pattern matching, 
    we can access a tuple element directly by using a period (.) followed by the index of the value we want to access. 
    For example:
    */
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{} _ {} _ {}", five_hundred, six_point_four, one);

    //// the array type ////
    //Unlike a tuple, every element of an array must have the same type.
    //arrays have a fixed length
    //allocate data on the stack rather than the heap
    let a = [1, 2, 3, 4, 5];
    println!("{:?}",a);
    //vectors are like arrays, but can grow/shrink, more on those in later stuffs
    /*You would write an array’s type by using square brackets, 
    and within the brackets include the type of each element, a semicolon, and then the number of elements in the array, like so:*/
    let arr: [i32; 5] = [10, 20, 30, 40, 50]; //Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.
    println!("{:?}",arr);
    /*if you want to create an array that contains the same value for each element, 
    you can specify the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:*/
    let a = [3; 5];
    println!("{:?}",a);
    /*The array named a will contain 5 elements that will all be set to the value 3 initially. 
    This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.*/
    //rust programs will crash if an out of index value is called
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered was not a number");
    let element = arr[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}
