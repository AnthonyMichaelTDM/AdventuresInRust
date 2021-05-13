#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn main() {
    let p = Coin::Penny;
    println!("a {:?} is worth {} cent(s)", p, value_in_cents(&p));

    let n = Coin::Nickel;
    println!("a {:?} is worth {} cent(s)", n, value_in_cents(&n));

    let d = Coin::Dime;
    println!("a {:?} is worth {} cent(s)", d, value_in_cents(&d));

    let q = Coin::Quarter(UsState::Alaska);
    println!("is worth {} cent(s)", value_in_cents(&q));
    let q = Coin::Quarter(UsState::Alabama);
    println!("is worth {} cent(s)", value_in_cents(&q));



    //matching with Option<T>
    /*we can also handle Option<T> using match as we did with the Coin enum!
    Instead of comparing coins, we’ll compare the variants of Option<T>, but the way that the match expression works remains the same.*/

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?}", six, none);

    //matches are exhaustive
    /*we must exhaust every last possibility in order for the code to be valid. 
    Especially in the case of Option<T>, when Rust prevents us from forgetting to explicitly handle the None case, 
    it protects us from assuming that we have a value when we might have null*/

    //The _ Placeholder
    let some_u8_value = 0u8;
    //The _ pattern will match any value. By putting it after our other arms, 
    //the _ will match all the possible cases that aren’t specified before it.
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), //The () is just the unit value, so nothing will happen in the _ case. 
    }
    //As a result, we can say that we want to do nothing for all the possible values that we don’t list before the _ placeholder.


    //However, the match expression can be a bit wordy in a situation in which we care about only one of the cases. 
    //For this situation, Rust provides if let. (next project)

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin { //sorta like a switch case, but MUCH more powerful
        Coin::Penny => { //arm with multiple lines of code?
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { //Patterns that Bind to Values
            //Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. 
            //This is how we can extract values out of enum variants.
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
