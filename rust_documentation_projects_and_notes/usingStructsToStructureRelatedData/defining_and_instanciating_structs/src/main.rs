
/*To define a struct, we enter the keyword struct and name the entire struct. 
A struct’s name should describe the significance of the pieces of data being grouped together. 
Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields. 
For example, Listing 5-1 shows a struct that stores information about a user account.*/
struct User {
    _username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
//we used the owned String type rather than the &str string slice type. 
//This is a deliberate choice because we want instances of this struct to own all of its data 
//and for that data to be valid for as long as the entire struct is valid.

fn main() {
    /*Structs are similar to tuples, which were discussed in /commonProgrammingConcepts/data_types. 
    Like tuples, the pieces of a struct can be different types. 
    Unlike with tuples, you’ll name each piece of data so it’s clear what the values mean. 
    As a result of these names, structs are more flexible than tuples: 
    you don’t have to rely on the order of the data to specify or access the values of an instance.*/

    //instanciating a struct
    let mut user1 = build_user_short(String::from("someone@example.com"),String::from("someusername123"));

    //to access values from structs, use dot notation, you can even change values in the struct is mutable
    user1.email = String::from("anotheremail@example.com");
    //Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable. 

    
    //creating instances from other instances
    //long and tedious
    let _user2 = User {
        email: String::from("another@example.com"),
        _username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    //using struct update syntax
    let _user2 = User {
        email: String::from("another@example.com"),
        _username: String::from("anotherusername567"),
        ..user1 //The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    };

    //Using Tuple Structs without Named Fields to Create Different Types
    /*Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; 
    rather, they just have the types of the fields. 
    Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples, 
    and naming each field as in a regular struct would be verbose or redundant.*/
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    /*Note that the black and origin values are different types, 
    because they’re instances of different tuple structs. 
    Each struct you define is its own type, even though the fields within the struct have the same types. 
    For example, a function that takes a parameter of type Color cannot take a Point as an argument, 
    even though both types are made up of three i32 values. Otherwise, tuple struct instances behave like tuples: 
    you can destructure them into their individual pieces, 
    you can use a . followed by the index to access an individual value, and so on.*/


    /*You can also define structs that don’t have any fields! 
    These are called unit-like structs because they behave similarly to (), the unit type. 
    Unit-like structs can be useful in situations in which you need to implement a trait on some type 
    but don’t have any data that you want to store in the type itself.*/



}


/*As with any expression, we can construct a new instance of the struct as the last expression
in the function body to implicitly return that new instance.*/
//long and tedious
fn _build_user(email: String, username: String) -> User {
    User {
        email: email,
        _username: username,
        active: true,
        sign_in_count: 1,
    }
}
//Using the Field Init Shorthand when Variables and Fields Have the Same Name
fn build_user_short(email: String, _username: String) -> User {
    User {
        email,
        _username,
        active: true,
        sign_in_count: 1,
    }
}

