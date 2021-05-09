use std::io; //this is how you bring other rust files, or external libraries into scope (similar to import in java)
             //this is bringing the 'io' library from the Standard Library (std) into scope

use rand::Rng;
use std::cmp::Ordering;

//the 'fn' syntax declares a function, the '()' are where parameters would go if applicable, and the '{' starts the body of the function
fn main() {
    println!("Guess the number!");

    //generate a number between 1 and 10 (could also pass the range (1..=10) for the same effect)
    let secret_number = rand::thread_rng().gen_range(1..11);
    println!("Secret number is: {}", secret_number);

    //loop, the match statement later will close the program when the use guesses correctly
    loop {
        println!("Please enter your guess");

        /*variables in rust are non-mutabable by default,
          to make a mutatable variable, add the 'mut' after 'let'*/
        let mut guess = String::new(); //more of this is discussed in Variables and Mutability in the rust docs

        /*using things from other rust files, or external libaries is similar to java, just use '::' instead of '.' .... sorta,
          :: points to a library, then function within the library (if applicable), . points to methods within the function*/
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line"); 
        /*get a line of input from the end user
        the '.expect()' is to handle potential errors, without this, code will still compile, but the compiler will give you a warning
        this usage of .expect() will print "failed to read line", then crash the program, more on error handling can be found later*/

        
        //convert the guess to a number so it can be compared with the secret number
        //this is an example of shadowing, were we change the type of a variable to reuse it
        let guess: u32 = match guess.trim().parse() { /*in these brackets, we use a match expression to handle invalid inputs rather than a .expect statement*/
            Ok(num) => num,
            Err(_) => continue, /*the _ is a catch all value, and 'continue' tells the program to end this loop iteration and start the next one*/
        };
        
        //that is the syntax for declaring a typed variable, here's it again but generalized
        //let [mut (optional)] variable_name: type = value;

        println!("You Guessed {}", guess);

        //comparing the guess to teh secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
