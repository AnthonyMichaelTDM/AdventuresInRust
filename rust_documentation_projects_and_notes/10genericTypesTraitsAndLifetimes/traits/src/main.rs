/*
 * Traits: Defining Shared Behavior
 * 
 * A trait tells the Rust compiler about functionality a particular type has and can share with other types. 
 * We can use traits to define shared behavior in an abstract way. 
 * We can use trait bounds to specify that a generic can be any type that has certain behavior.
 * Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
*/

/*
 * prompt for this library
 * For example, let’s say we have multiple structs that hold various kinds and amounts of text: 
 * a NewsArticle struct that holds a news story filed in a particular location 
 * and a Tweet that can have at most 280 characters 
 * along with metadata that indicates whether it was a new tweet, a retweet, or a reply to another tweet.
 * 
 * We want to make a media aggregator library that can display summaries of data that might be stored in a NewsArticle or Tweet instance. 
 * To do this, we need a summary from each type, and we need to request that summary by calling a summarize method on an instance.
*/








////////////////////
//defining a trait//
////////////////////

/*
 * A type’s behavior consists of the methods we can call on that type. 
 * Different types share the same behavior if we can call the same methods on all of those types. 
 * Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.
*/
pub trait Summary { //here, we declare a trait using the trait keyword and then the trait’s name, which is Summary in this case
    fn summarize(&self) -> String;  //we declare the method signatures that describe the behaviors of the types that implement this trait,
                                    //which in this case is fn summarize(&self) -> String
                                    //After the method signature, instead of providing an implementation within curly brackets, we use a semicolon.
                                    //Each type implementing this trait must provide its own custom behavior for the body of the method. 
                                    //The compiler will enforce that any type that has the Summary trait will have the method summarize defined with this signature exactly.
                                    
                                    //A trait can have multiple methods in its body: 
                                    //the method signatures are listed one per line and each line ends in a semicolon.
}





//////////////////////////////////
//Implementing a Trait on a Type//
//////////////////////////////////
/*
 * Now that we’ve defined the desired behavior using the Summary trait, we can implement it on the types in our media aggregator.
 */
pub struct NewsArticle { // declaring a structure
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle { // assigning it a trait
    fn summarize(&self) -> String { // defining implementation of the traits behaviors
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet { // declaring a structure
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet { // assigning it a trait
    fn summarize(&self) -> String { // defining implementation of the traits behaviors
        format!("{}: {}", self.username, self.content)
    }
}

/*
 * One restriction to note with trait implementations is that we can implement a trait on a type only if either 
 * the trait or the type is local to our crate. 
 * For example, we can implement standard library traits like Display on a custom type like Tweet as part of our aggregator crate functionality, 
 * because the type Tweet is local to our aggregator crate. 
 * We can also implement Summary on Vec<T> in our aggregator crate, because the trait Summary is local to our aggregator crate.
 *
 * But we can’t implement external traits on external types. 
 * For example, we can’t implement the Display trait on Vec<T> within our aggregator crate, 
 * because Display and Vec<T> are defined in the standard library and aren’t local to our aggregator crate. 
 * This restriction is part of a property of programs called coherence, and more specifically the orphan rule, 
 * so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. 
 * Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.
*/





///////////////////////////
//Default Implementations//
///////////////////////////
/*
 * Sometimes it’s useful to have default behavior for some or all of the methods in a trait 
 * instead of requiring implementations for all methods on every type. 
 * Then, as we implement the trait on a particular type, we can keep or override each method’s default behavior.
 *
*/
pub trait Summary2 {
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }
}
//To use a default implementation instead of defining a custom implementation, we specify an empty impl block, for example:
impl Summary2 for NewsArticle {}
/* Even though we’re no longer defining the summarize2 method on NewsArticle directly, 
 * we’ve provided a default implementation and specified that NewsArticle implements the Summary2 trait. 
 * As a result, we can still call the summarize2 method on an instance of NewsArticle
*/

/*
 * The syntax for overriding a default implementation is the same as
 * the syntax for implementing a trait method that doesn’t have a default implementation.
 * 
 * Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation. 
 * In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it.
*/
pub trait Summary3 {
    fn summarize_author(&self) -> String;

    fn summarize3(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
} //To use this version of Summary, we only need to define summarize_author when we implement the trait on a type:
impl Summary3 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}




////////////////////////
//Traits as parameters//
////////////////////////

//use traits to define functions that accept many different types.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
} 
/*
 * Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name. 
 * This parameter accepts any type that implements the specified trait. 
 * In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize.
 * We can call notify and pass in any instance of NewsArticle or Tweet. 
 * Code that calls the function with any other type, such as a String or an i32, won’t compile because those types don’t implement Summary.
*/

//Trait Bound Syntax
/*
 * The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a trait bound;

 pub fn notify<T: Summary>(item: &T) {
   println!("Breaking news! {}", item.summarize());
 }

 * This longer form is equivalent to the example in the previous section but is more verbose. 
 * We place trait bounds with the declaration of the generic type parameter after a colon and inside angle brackets.
 * 
 * The impl Trait syntax is convenient and makes for more concise code in simple cases. 
 * If we wanted this function to allow item1 and item2 to have different types, 
 * using impl Trait would be appropriate (as long as both types implement Summary) like this:

 pub fn notify(item1: &impl Summary, item2: &impl Summary) {//code//}
  
 * If we wanted to force both parameters to have the same type, that’s only possible to express using a trait bound, like this:

 pub fn notify<T: Summary>(item1: &T, item2: &T) {//code//}

 * The generic type T specified as the type of the item1 and item2 parameters constrains the function such that the concrete type 
 * of the value passed as an argument for item1 and item2 must be the same.
*/

//multiple trait bounds
/*
 * We can also specify more than one trait bound using the '+' syntax, 
 * for example, this specifies a function whos parameter, item, must have both the Summary and Display traits 

 pub fn notify(item: &(impl Summary + Display)) {}

 * The + syntax is also valid with trait bounds on generic types:

 pub fn notify<T: Summary + Display>(item: &T) {}

 * 
 * Clearer Trait Bounds with where Clauses
 * Using too many trait bounds has its downsides. Each generic has its own trait bounds, 
 * so functions with multiple generic type parameters can contain lots of trait bound information 
 * between the function’s name and its parameter list, making the function signature hard to read. 
 * For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature. 
 * So instead of writing this:

 fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

 * we can use a where clause, like this:

 fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
 {

 * This function’s signature is less cluttered: the function name, parameter list, and return type are close together, 
 * similar to a function without lots of trait bounds.
*/

/*
 * Returning Types that Implement Traits
 * We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait, as shown here:
*/
fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
/*
 * The ability to return a type that is only specified by the trait it implements is especially useful in the context of 
 * closures and iterators, which we cover in Chapter 13. Closures and iterators create types that only the compiler knows 
 * or types that are very long to specify. The impl Trait syntax lets you concisely specify that a function returns some type
 * that implements the Iterator trait without needing to write out a very long type.
 * 
 * However, you can only use impl Trait if you’re returning a single type. 
 * 
 * For example, this code that returns either a NewsArticle or a Tweet with the return type specified as impl Summary wouldn’t work:
 fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
 }
 * We’ll cover how to write a function with this behavior in the “Using Trait Objects That Allow for Values of Different Types” section of Chapter 17.
*/

//here is a fixed version of the largest function used in the types stuff
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}





//////////////////////////////////////////////////////////////
// conditional implementation of methods using trait bounds //
//////////////////////////////////////////////////////////////

/*
 * By using a trait bound with an impl block that uses generic type parameters, 
 * we can implement methods conditionally for types that implement the specified traits. 
 * 
 * For example, the type Pair<T> below always implements the new function. 
 * But Pair<T> only implements the cmp_display method if 
 * its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.
*/
use std::fmt::Display;
struct _Pair<T> {
    x: T,
    y: T,
}
impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
/*
 * We can also conditionally implement a trait for any type that implements another trait. 
 * Implementations of a trait on any type that satisfies the trait bounds are called 
 * blanket implementations and are extensively used in the Rust standard library. 
 * For example, the standard library implements the ToString trait on any type that implements the Display trait.
 */










fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
    println!("New article available! {}", article.summarize2());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize3());


    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}