/* Generic Data Types
 * 
 * We can use generics to create definitions for items like function signatures or structs, 
 * which we can then use with many different concrete data types. 
 * Let’s first look at how to define functions, structs, enums, and methods using generics. 
 * Then we’ll discuss how generics affect code performance.
*/




////////////////////////////
//generics in definitions://
////////////////////////////

//in function definititions://

/* rather than having duplicate functions for every concievable data type, like this:
 fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {    if item > largest { largest = item; }    }
    largest
 }
 fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {    if item > largest { largest = item; }    }
    largest
 *}
 *
 *simply use a generic type!
 *
 *To parameterize the types in the new function we’ll define, we need to name the type parameter, 
 *just as we do for the value parameters to a function
 *
 *When we use a parameter in the body of the function, 
 *we have to declare the parameter name in the signature so the compiler knows what that name means. 
 *Similarly, when we use a type parameter name in a function signature, 
 *we have to declare the type parameter name before we use it. 
 *To define the generic largest function, place type name declarations inside angle brackets, <>, 
 *between the name of the function and the parameter list */
//example
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {    if item > largest { largest = item; }    }
//     largest
// } 
//this won't compile because '>' can't be used for every conceivable type, we'll learn how to fix this with traits later





//in struct definitions
/*We can also define structs to use a generic type parameter in one or more fields using the <> syntax*/
/*
 * The syntax for using generics in struct definitions is similar to that used in function definitions. 
 * First, we declare the name of the type parameter inside angle brackets just after the name of the struct. 
 * Then we can use the generic type in the struct definition where we would otherwise specify concrete data types.
*/
//with one generic type, a struct might look something like this
struct PointWithOneGeneric<T> {
    x: T,
    y: T,
} //with this, x and y must be the same type or you'll get an error, 
struct PointWithMultipleGenerics<T, U> {
    x: T,
    y: U,
}
/*but, you can use multiple generic types in the same struct,
 *for example,
 *To define a Point struct where x and y are both generics but could have different types, we can use multiple generic type parameters.
*/
//btw, you can have more than 2 generics too, there isn't really a limit.





//in enum declarations
//basically the same as structs tbh





//In Method Definitions

//We can implement methods on structs and enums (as we did in Chapter 5) and use generic types in their definitions, too. 
//example for one generic
impl<T> PointWithOneGeneric<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
//when you want a method to only be available to structs/enums of a specific type, you only need to have <> after the struct name, with the 
//type in the <>'s, for example:
impl PointWithOneGeneric<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
//of course, you can also use multiple generic types with methods, for example:
impl<T, U> PointWithMultipleGenerics<T,U> {
    //additionally, Generic type parameters in a struct definition 
    //aren’t always the same as those you use in that struct’s method signatures.
    //(just make sure you're consistent
    fn mixup<V,W> (self, other: PointWithMultipleGenerics<V, W>) -> PointWithMultipleGenerics<T,W> {
        PointWithMultipleGenerics {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let both_integer = PointWithOneGeneric { x: 5, y: 10 };
    let both_float = PointWithOneGeneric { x: 1.0, y: 4.0 };
    let integer_and_float = PointWithMultipleGenerics { x: 5, y: 4.0 };

    let p1 = both_integer;
    println!("p1.x = {}", p1.x());

    let p2 = both_float;
    println!("p2's distance from origin is: {}", p2.distance_from_origin());

    
    let p3 = integer_and_float;
    let p4 = PointWithMultipleGenerics {x: "Hello", y: 'C'};
    let p5 = p3.mixup(p4);
    println!("p5.x = {}, p5.y = {}", p5.x, p5.y);
}






//////////////////////////////////////
//Performance of Code Using Generics//
//////////////////////////////////////



/* You might be wondering whether there is a runtime cost when you’re using generic type parameters. */  
/* short answer: no */
/* long answer:
 * The good news is that Rust implements generics in such a way that your code doesn’t run any slower using generic types 
 * than it would with concrete types.
 * 
 * 
 * Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. 
 * Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
 * 
 * Monomorphization baiscally turns something like this:
 enum Option<T> {Some<T>, none,}
 fn main() {
    let integer = Some(5);
    let float = Some(5.0);
 }
 * into something like this
 enum Option_i32 { Some(i32),None, }
 enum Option_f64 { Some(f64),None, }
 fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
 }
 * during compilation 
 * 
 * 
 * Because Rust compiles generic code into code that specifies the type in each instance, 
 * we pay no runtime cost for using generics. When the code runs, 
 * it performs just as it would if we had duplicated each definition by hand. 
 * The process of monomorphization makes Rust’s generics extremely efficient at runtime.
 */