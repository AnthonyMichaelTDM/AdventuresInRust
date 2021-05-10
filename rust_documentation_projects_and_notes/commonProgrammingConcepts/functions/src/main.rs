fn main() {
    /*Rust code uses snake case as the conventional style for function and variable names. 
    In snake case, all letters are lowercase and underscores separate words */
    println!("Hello, world!");

    another_function();

    parameters_example(32);

    multiple_parameters_example(135,-814);


    //statesments vs. expressions
    //Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.
    /*
    for example, variable declarations are statements,
    however, math operations are expressions
    Calling a function is an expression. Calling a macro is an expression. The block that we use to create new scopes, {}, is an expression,
    */
    //for example
    let y = {
        let x = 3;
        x + 1 //no semicolon makes this an expression
    };
    /*This expression is a block that, in this case, evaluates to 4. 
    That value gets bound to y as part of the let statement. 
    Note the x + 1 line without a semicolon at the end, which is unlike most of the lines you’ve seen so far. 
    Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, 
    you turn it into a statement, which will then not return a value. 
    Keep this in mind as you explore function return values and expressions next.*/
    println!("The value of y is: {}", y);

    println!("the value returned by the function is {}", return_example());


}

//first example function, just to show the conventions for naming functions (snake_case)
fn another_function() {
    println!("Another function.");
}

/*Functions can also be defined to have parameters, 
which are special variables that are part of a function’s signature. 
When a function has parameters, you can provide it with concrete values for those parameters. 
Technically, the concrete values are called arguments, but in casual conversation, 
people tend to use the words parameter and argument interchangeably 
for either the variables in a function’s definition or the concrete values passed in when you call a function.*/
//In function signatures, you MUST declare the type of each parameter.
fn parameters_example(x: i32) { 
    println!("The value of x is: {}", x);
}

//when you want to have multiple parameters, separate them with commas
fn multiple_parameters_example(x: i32, y: i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}


//Functions can return values to the code that calls them. We don’t name return values, but we do declare their type after an arrow (->)
fn return_example() -> i32 {
    let mut x = 5;
    x *= 20;  
    x //as long as the function ends in an expression, it's valid
}