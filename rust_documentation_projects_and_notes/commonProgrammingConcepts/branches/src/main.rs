fn main() {
    //basic if/else
    let number = 3;
    if number < 5 { //condition must be a boolean expression
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //handling multiple conditions with elseif
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //Using if in a let Statement
    //Because if is an expression, we can use it on the right side of a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    //Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions. 
    //In this case, the value of the whole if expression depends on which block of code executes. 
    //This means the values that have the potential to be results from each arm of the if must be the same type
    println!("The value of number is: {}", number);

    //Using too many else if expressions can clutter your code, 
    //so if you have more than one, you might want to refactor your code. 
    //Chapter 6 describes a powerful Rust branching construct called match for these cases.



    ////// repitition with loops //////
    //rust has 3 kinds of loops, 'loop', 'while', and 'for'

    ////repeating code with loop////
    //The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //exits the loop and returns counter*2
        }
    };
    println!("The result is {}", result);

    ////conditional loops with while////
    //it's possible to do this with a 'loop', but using 'while' is cleaner, more concise, and less error prone
    //While a condition holds true, the code runs; otherwise, it exits the loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");

    ////looping through a collection with for////
    //possible with a 'while', but using a 'for' is cleaner, more concise, and less error prone
    //execute some code for each item in a collection.
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    //even in most situations where a while loop is apporpriate, a for loop can do the same thing, in a safer manner
    for number in (1..4).rev() { //parse through every number in the range 1-4 (reversed (so, 4-1))
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");



    
}
