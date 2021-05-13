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

/**
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
/**Even though we’re no longer defining the summarize2 method on NewsArticle directly, 
 * we’ve provided a default implementation and specified that NewsArticle implements the Summary2 trait. 
 * As a result, we can still call the summarize2 method on an instance of NewsArticle
*/

/**
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
/** */

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
}