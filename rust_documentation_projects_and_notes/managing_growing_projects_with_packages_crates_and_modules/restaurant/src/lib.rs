mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
pub fn eat_at_front_restaurant1() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}//Adding the pub keyword to mod hosting and fn add_to_waitlist lets us call the function from eat_at_restaurant

//Adding use and a path in a scope is similar to creating a symbolic link in the filesystem.
use crate::front_of_house::hosting;
pub fn eat_at_front_restaurant2() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}//Bringing a module into scope with use





fn serve_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); //super
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub fn eat_at_back_restaurant1() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
pub fn eat_at_back_restaurant2() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}//Designating an enum as public makes all its variants public



//idiomatic use paths
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}// Bringing HashMap into scope in an idiomatic way

//The Glob Operator
//If we want to bring all public items defined in a path into scope, we can specify that path followed by *, the glob operator:
use std::collections::*;


////////summary
/*Summary

Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module. 
You can do this by specifying absolute or relative paths. 
These paths can be brought into scope with a use statement so you can use a shorter path for multiple uses of the item in that scope. 
Module code is private by default, but you can make definitions public by adding the pub keyword.


In the next chapter, we’ll look at some collection data structures in the standard library that you can use in your neatly organized code.*/