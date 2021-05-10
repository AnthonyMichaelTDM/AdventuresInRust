use std::io;
fn main() {
    let mut n = String::new();

    //read input from user
    println!("enter the term");
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: u8 = n.trim().parse().expect("n entered was not a number");

    //count up the sequence until the value the user entered
    for number in 1..n {
        //if number%2 == 0 {
        //println!("{}th term of the fibonacci sequence is {} (recursion)",number, _nth_fibonacci_recursion(number)); 
        //} 
        //else {
        println!("{}th term of the fibonacci sequence is {} (loop)",number, _nth_fibonacci_loop(number)); //loops are wayy faster
        //}
    }
}

//using recursion
fn _nth_fibonacci_recursion(n: u8) -> usize {
    let term: usize;
    if n < 2 {
        term = 1;
    } else {
        term = _nth_fibonacci_recursion(n-1) + _nth_fibonacci_recursion(n-2);
    }
    term
}

//using a loop
fn _nth_fibonacci_loop(n: u8) -> u128 {
    let mut n_minus_1: u128 = 1;
    let mut n_minus_2: u128;
    let mut term: u128 = 1;
    for _number in 1..n {
        n_minus_2 = n_minus_1;
        n_minus_1 = term;
        term = n_minus_1 + n_minus_2;
    }
    term
}