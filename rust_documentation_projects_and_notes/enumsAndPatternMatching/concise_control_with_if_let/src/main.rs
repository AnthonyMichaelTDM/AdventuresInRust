fn main() {
    //the following 2 code chunks do the exact same thing, one using a match, the other using if let
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    
    //using if let, you gain conciseness at the cost of exhaustive checking


    //the following 2 code chunks also do the same thing, one using match, the other using if let

    let coin = Coin::Nickel;
    let mut _count = 0;
    match coin {
        Coin::_Quarter(state) => println!("State quarter from {:?}!", state),
        _ => _count += 1,
    }

    let coin = Coin::Nickel;
    let mut _count = 0;
    if let Coin::_Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        _count += 1;
    }

    //If you have a situation in which your program has logic that is too verbose to express using a match, 
    //remember that if let is in your Rust toolbox as well.
}


#[derive(Debug)]
enum Coin {
    _Penny,
    Nickel,
    _Dime,
    _Quarter(UsState),
}
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    _Alabama,
    _Alaska,
    // --snip--
}
