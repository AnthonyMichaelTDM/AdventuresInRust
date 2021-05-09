fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6; //this won't work unless x is set as mutable in its declaration
    println!("The value of x is {}", x);

    //////difference between variables and constants//////
    
    // you can't use the 'mut' keyword with constants, they are always immutable
    // you must give a const a type,
    // const's can be declared in any scope
    const MAX_POINTS: u32 = 100_000; //common convention is for the names of constants to be all uppercase
    // underscores can be insterted into numeric literals to improve readability
    println!("Max points: {}", MAX_POINTS);
    /*
    Constants are valid for the entire time a program runs, 
    within the scope they were declared in, 
    making them a useful choice for values in your application domain that multiple parts of the program might need to know about, 
    such as the maximum number of points any player of a game is allowed to earn or the speed of light.

    Naming hardcoded values used throughout your program as constants is useful 
    in conveying the meaning of that value to future maintainers of the code. 
    It also helps to have only one place in your code you would need to change 
    if the hardcoded value needed to be updated in the future.
    */


    //////Shadowing//////
    //you can declare a new variable with the same name as a previous variable. 
    /*Rustaceans say that the first variable is shadowed by the second, 
    which means that the second variable’s value is what appears when the variable is used. 
    We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows: */
    let y = 5;

    let y = y + 1;

    let y = y * 3;

    println!("The value of y is {}", y);

    //shadowing is useful when you need to change the type of a variable without declaring a new variable
    //or when you want to change the value of an immutable variable
    
    //for example, something like this isn't possible by just using a mutable variable as the type of 'spaces' changes
    let spaces = "     ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);


}
