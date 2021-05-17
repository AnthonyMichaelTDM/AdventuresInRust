////////////////////////////////////////
//Validating References with Lifetimes//
////////////////////////////////////////
/*
 * Another kind of generic that we’ve already been using is called lifetimes.
 * Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.
 *
 * every reference in Rust has a lifetime, which is the scope for which that reference is valid.
 * Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.
 * We must annotate types when multiple types are possible.
 * In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways.
 * Rust requires us to annotate the relationships using generic lifetime parameters to ensure
 * the actual references used at runtime will definitely be valid.
 *
 * Preventing Dangling References with Lifetimes
 *
 * The main aim of lifetimes is to prevent dangling references,
 * which cause a program to reference data other than the data it’s intended to reference.
 * Consider the program below, which has an outer scope and an inner scope.
 {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
 }
 * The outer scope declares a variable named r with no initial value,
 * and the inner scope declares a variable named x with the initial value of 5.
 * Inside the inner scope, we attempt to set the value of r as a reference to x.
 * Then the inner scope ends, and we attempt to print the value in r.
 * This code won’t compile because the value r is referring to has gone out of scope before we try to use it.
 *
 *
 * So how does Rust determine that this code is invalid? It uses a borrow checker.
 *
 *
 * The Borrow Checker
 *
 * The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.
   {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
 * Here, we’ve annotated the lifetime of r with 'a and the lifetime of x with 'b.
 * As you can see, the inner 'b block is much smaller than the outer 'a lifetime block.
 * At compile time, Rust compares the size of the two lifetimes and sees
 * that r has a lifetime of 'a but that it refers to memory with a lifetime of 'b.
 * The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.
 *
 * fixed the code so it doesn’t have a dangling reference and compiles without any errors.
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
 * Here, x has the lifetime 'b, which in this case is larger than 'a.
 * This means r can reference x because Rust knows that the reference in r will always be valid while x is valid.
*/

////////////////////////////////////
// Generic Lifetimes in functions //
////////////////////////////////////
/*
 * we want the function to take string slices, which are references,
 * because we don’t want the longest function to take ownership of its parameters.
 fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
 }
 * but this doesn't work, because the compiler can't infer the lifetime of it's output because the output changes,
 * and the 2 possible outputs don't neccessarily have the same lifetimes
 *
 * When we’re defining this function, we don’t know the concrete values that will be passed into this function,
 * so we don’t know whether the if case or the else case will execute.
 * We also don’t know the concrete lifetimes of the references that will be passed in,
 * so we can’t look at the scopes to determine whether the reference we return will always be valid.
 * The borrow checker can’t determine this either,
 * because it doesn’t know how the lifetimes of x and y relate to the lifetime of the return value.
 * To fix this error, we’ll add generic lifetime parameters that define the relationship between the references
 * so the borrow checker can perform its analysis.
 *
*/
/* Lifetime Annotations in Function Signatures
 *
 * Now let’s examine lifetime annotations in the context of the longest function.
 * As with generic type parameters, we need to declare generic lifetime parameters inside angle brackets between the function name
 * and the parameter list.
 * The constraint we want to express in this signature is that all the references
 * in the parameters and the return value must have the same lifetime.
 * We’ll name the lifetime 'a and then add it to each reference, as shown below
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //similar to generic types, the character you assign a lifetime doesn't really matter, as long as you're consistant
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/* The function signature now tells Rust that for some lifetime 'a, the function takes two parameters,
 * both of which are string slices that live at least as long as lifetime 'a.
 * The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a.
 * In practice, it means that the lifetime of the reference returned by the longest function is the same as the
 * smaller of the lifetimes of the references passed in.
 *
 * Remember, when we specify the lifetime parameters in this function signature,
 * we’re not changing the lifetimes of any values passed in or returned.
 * Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints.
 *
 * When annotating lifetimes in functions, the annotations go in the function signature, not in the function body.
 *
 * When we pass concrete references to longest,
 * the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y.
 * In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y.
 * Because we’ve annotated the returned reference with the same lifetime parameter 'a,
 * the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.
*/
/* thinking in terms of lifetimes
 *
 * The way in which you need to specify lifetime parameters depends on what your function is doing,
 * When returning a reference from a function,
 * the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
 * If the reference returned does not refer to one of the parameters, it must refer to a value created within this function,
 * which would be a dangling reference because the value will go out of scope at the end of the function.
 * Consider this attempted implementation of the longest function that won’t compile:
 fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
 }
 * Here, even though we’ve specified a lifetime parameter 'a for the return type,
 * this implementation will fail to compile because the return value lifetime is not related to the lifetime of the parameters at all.
 *
 * the issue is that 'result' goes out of scope and is cleaned up at the end of the function,
 * meaning that the function is attempting to return a dangling reference, which is not allowed.
 *
 * there is no way to specify lifetime parameters that would change a dangling reference,
 * in this case it's best to return an owned data type rather than a reference so that the calling function is responsible for cleanup.
*/
// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.
// Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create
// dangling pointers or otherwise violate memory safety.

////////////////////////////////
// lifetime annotation syntax //
////////////////////////////////
/*
 * Lifetime annotations don’t change how long any of the references live.
 * Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
 *
 * Just as functions can accept any type when the signature specifies a generic type parameter,
 * functions can accept references with any lifetime by specifying a generic lifetime parameter.
 *
 * syntax: the names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short,
 * like generic types. Most people use the name 'a. We place lifetime parameter annotations after the & of a reference,
 * using a space to separate the annotation from the reference’s type.
 *
 * examples:
 &i32        // a reference
 &'a i32     // a reference with an explicit lifetime
 &'a mut i32 // a mutable reference with an explicit lifetime
 *
 * One lifetime annotation by itself doesn’t have much meaning,
 * because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other.
*/

///////////////////////////////////////////////
// lifetime annotations in struct definitions//
///////////////////////////////////////////////
/*
 * So far, we’ve only defined structs to hold owned types. It’s possible for structs to hold references,
 * but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.
 *
 * for example, while this isn't valid:
 struct ImportantExcerpt {
     part: &str,
 }
 * this is:
*/
struct ImportantExcerpt<'a> {
    _part: &'a str,
}

///////////////////////////////////////////////
// lifetime annotation in method definitions //
///////////////////////////////////////////////
/*
 * when we implement methods on a struct with lifetimes, we use the same syntax as that of generic type parameters.
 *
 * Where we declare and use the lifetime parameters depends on whether they’re related to the struct fields
 * or the method parameters and return values.
 *
 * Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name,
 * because those lifetimes are part of the struct’s type.
 *
 * In method signatures inside the impl block, references might be tied to the lifetime of references in the struct’s fields,
 * or they might be independent.
 *
 * In addition, the lifetime elision rules often make it so that lifetime annotations aren’t necessary in method signatures.
*/
//some examples
impl<'a> ImportantExcerpt<'a> {
    //First, we’ll use a method named level whose only parameter is a reference to self and whose return value is an i32,
    //which is not a reference to anything:
    fn _level(&self) -> i32 {
        //The lifetime parameter declaration after impl and its use after the type name are required,
        //but we’re not required to annotate the lifetime of the reference to self because of the first elision rule.
        3
    }

    //here is an example where the third lifetime elision rule applies 
    fn _announce_and_return_part(&self, announcement: &str) -> &str {
        //There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. 
        //Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.
        println!("Attention please: {}", announcement);
        self._part
    }
}

//////////////////////
// lifetime elision //
//////////////////////
//there are some cases where declaring lifetimes isn't neccessary because the compiler has rules to add them
/*
 * The first rule is that each parameter that is a reference gets its own lifetime parameter.
 * In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
 * a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
 *
 * The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
 * fn foo<'a>(x: &'a i32) -> &'a i32.
 *
 * The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
 * the lifetime of self is assigned to all output lifetime parameters.
 * This third rule makes methods much nicer to read and write because fewer symbols are necessary.
*/




/////////////////////////
// The Static Lifetime //
/////////////////////////

/* 
 * One special lifetime we need to discuss is 'static, which means that this reference can live for the entire duration of the program. 
 * All string literals have the 'static lifetime, which we can annotate as follows:
 let s: &'static str = "I have a static lifetime.";

 * The text of this string is stored directly in the program’s binary, which is always available. 
 * Therefore, the lifetime of all string literals is 'static.
*/
//You might see suggestions to use the 'static lifetime in error messages. 
//But before specifying 'static as the lifetime for a reference, 
//think about whether the reference you have actually lives the entire lifetime of your program or not. 
//You might consider whether you want it to live that long, even if it could. 

//Most of the time, the problem results from attempting to create a dangling reference or a mismatch of the available lifetimes. 
//In such cases, the solution is fixing those problems, not specifying the 'static lifetime.



/////////////////////////////////////////////////////////////////
//Generic Type Parameters, Trait Bounds, and Lifetimes Together//
/////////////////////////////////////////////////////////////////

//Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!
use std::fmt::Display;
fn _longest_with_an_announcement<'a, T>( // generic types
    x: &'a str, //lifetimes
    y: &'a str, //lifetimes 
    ann: T, //trait bounds
) -> &'a str
where
    T: Display, //trait bounds
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/*
 * This is the longest function from Listing 10-22 that returns the longer of two string slices. 
 * But now it has an extra parameter named ann of the generic type T, 
 * which can be filled in by any type that implements the Display trait as specified by the where clause. 
 * This extra parameter will be printed before the function compares the lengths of the string slices, 
 * which is why the Display trait bound is necessary. Because lifetimes are a type of generic, 
 * the declarations of the lifetime parameter 'a and the generic type parameter T 
 * go in the same list inside the angle brackets after the function name.
*/






fn main() {
    println!("Hello, world!");

    //the signature of longest specifies that what it returns will be in the scope of the most specific of its two parameters
    //this works because the parameters have the same lifetime
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // this works, because although the paraters have different lifetimes,
    // the value returned has the same lifetime as the smallest lifetimes amoung its parameters
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    //this doesn't work
    // the reason being that while the call to longest is technically valid, the usage of the value returned is not
    // result has the lifetime of string 2, because string 2 has the smallest lifetime amoung parameters of longest
    // therefore, by the time it's used by the return statement, result is out of scope, its lifetime is over
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    //however, this does work, although intuitively the value assigned to result should be out of scope,
    // because it was defined concretely, its lifetime is that of its parameters, which are in the same scope as the usage of result,
    // therefore it is valid
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        _part: first_sentence,
    };
}




/* SUMMARY
 * 
 * Generic type parameters let you apply the code to different types. 
 * Traits and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs. 
 * You learned how to use lifetime annotations to ensure that this flexible code won’t have any dangling references. 
 * And all of this analysis happens at compile time, which doesn’t affect runtime performance!
 * 
*/