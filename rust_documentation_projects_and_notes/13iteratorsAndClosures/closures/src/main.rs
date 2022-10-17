/* rust closures are annonymous functions you can save in a variable or pass as arguments to other functions
 * 
 * You can create the closure in one place and then call the closure to evaluate it in a different context
 * 
 * Unlike functions, closures can capture values from the scope in which they’re defined.
 * 
 * 
     * closure syntax:
         * basic syntax 
        let closure_name = | params, separated, with, commas | {
            //closure code
        }; //note the semicolon, it's needed
         * catcher syntax
 * 
 * while rust will ussually be able to infer the data type of the parameter and returned value, you can explicitly define these 
 * with the syntax shown below
 * 
    let closure_name = | params: type, separated_by_commas: type| -> ReturnType {
        //closure code
    }; //note the semicolon, it's needed
 * 
 * the syntax for a closure is very similar to that of functions
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }      The first line shows a function definition, 
    let add_one_v2 = |x: u32| -> u32 { x + 1 };     and the second line shows a fully annotated closure definition. 
    let add_one_v3 = |x|             { x + 1 };     The third line removes the type annotations from the closure definition, 
    let add_one_v4 = |x|               x + 1  ;     and the fourth line removes the brackets, which are optional because the closure body has only one expression.
 * These are all valid definitions that will produce the same behavior when they’re called. 
 * Calling the closures is required for add_one_v3 and add_one_v4 to be able to compile
 * because the types will be inferred from their usage.
 * 
 * 
 * 
 * 
*/
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

fn main() {
    /////////////////////////////////////////////////////
    //Creating an Abstraction of Behavior with Closures//
    /////////////////////////////////////////////////////
    
    /*
     * Consider this hypothetical situation: we work at a startup that’s making an app to generate custom exercise workout plans. 
     * The backend is written in Rust, and the algorithm that generates the workout plan takes into account many factors, 
     * such as the app user’s age, body mass index, exercise preferences, recent workouts, and an intensity number they specify.
     * The actual algorithm used isn’t important in this example; what’s important is that this calculation takes a few seconds. 
     * We want to call this algorithm only when we need to and only call it once so we don’t make the user wait more than necessary.
     * 
     * We’ll simulate calling this hypothetical algorithm with the function simulated_expensive_calculation
     * 
     * Next is the main function, which contains the parts of the workout app important for this example. 
     * This function represents the code that the app will call when a user asks for a workout plan.
     * Because the interaction with the app’s frontend isn’t relevant to the use of closures, 
     * we’ll hardcode values representing inputs to our program and print the outputs.
     * 
     * The required inputs are these:
         * An intensity number from the user, which is specified when they request a workout to indicate whether they want a low-intensity workout or a high-intensity workout
         * A random number that will generate some variety in the workout plans
     * 
    */
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    
    let pre_time = SystemTime::now();
    generate_workout(simulated_user_specified_value, simulated_random_number);
    println!("{} seconds", SystemTime::now().duration_since(pre_time).expect("time error").as_secs_f32());

    let pre_time = SystemTime::now();
    generate_workout_functions(simulated_user_specified_value, simulated_random_number);
    println!("{} seconds", SystemTime::now().duration_since(pre_time).expect("time error").as_secs_f32());

    let pre_time = SystemTime::now();
    generate_workout_closure(simulated_user_specified_value, simulated_random_number);
    println!("{} seconds", SystemTime::now().duration_since(pre_time).expect("time error").as_secs_f32());

    let pre_time = SystemTime::now();
    generate_workout_closure_cacher(simulated_user_specified_value, simulated_random_number);
    println!("{} seconds", SystemTime::now().duration_since(pre_time).expect("time error").as_secs_f32());
    

    //tests for the second cacher

    //show that it can cache more than one value, and that it doesn't redo calculations every time
    let mut expensive_closure_1 = Cacher2::new(|num| { 
        println!("calculating slowly");
        thread::sleep(Duration::from_secs(1));
        num
    });
    let pre_time = SystemTime::now();
    println!("{}, took {} seconds", expensive_closure_1.value(1), SystemTime::now().duration_since(pre_time).expect("time error").as_secs_f32());
    let pre_time = SystemTime::now();
    println!("{}, took {} seconds", expensive_closure_1.value(3), SystemTime::now().duration_since(pre_time).expect("time error").as_secs_f32());
    let pre_time = SystemTime::now();
    println!("{}, took {} seconds", expensive_closure_1.value(1), SystemTime::now().duration_since(pre_time).expect("time error").as_secs_f32());
    let pre_time = SystemTime::now();
    println!("{}, took {} seconds", expensive_closure_1.value(3), SystemTime::now().duration_since(pre_time).expect("time error").as_secs_f32());

    //show that it works with closures that return values of types different than their argument
    let mut length_closure = Cacher2::new(|val: &str| -> usize {
        println!("calculating slowly");
        thread::sleep(Duration::from_secs(1));
        val.len()
    });
    let pre_time = SystemTime::now();
    println!("{}, took {} seconds", length_closure.value("Hello"), SystemTime::now().duration_since(pre_time).expect("time error").as_secs_f32());
    let pre_time = SystemTime::now();
    println!("{}, took {} seconds", length_closure.value("World!"), SystemTime::now().duration_since(pre_time).expect("time error").as_secs_f32());
    let pre_time = SystemTime::now();
    println!("{}, took {} seconds", length_closure.value("Hello"), SystemTime::now().duration_since(pre_time).expect("time error").as_secs_f32());
    let pre_time = SystemTime::now();
    println!("{}, took {} seconds", length_closure.value("World!"), SystemTime::now().duration_since(pre_time).expect("time error").as_secs_f32());








}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

//this is the first, and worst, way to do this
//there is 1 major issue here, which is that the expendive calculation is called and ran 3 separate times, 
//when it only needs to be run once or not at all (depending on the inputs)  
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            simulated_expensive_calculation(intensity)
        );
    }
}

// first, we will refactor the above function using a variable to store the value of the calculation at the start, 
// therefore only calculating once
// issue with this is that we still may be calling unneccessarily as not all cases require the calculation
fn generate_workout_functions(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", expensive_result);
    }
}

//We want to refer to simulated_expensive_calculation only once in generate_workout, 
//but defer the expensive calculation to only where we actually need the result. This is a use case for closures!

//sooo ... we now refactor to use a closure
// issue with this is that now we're back to square one, we're still doing the calculation twice in the first branch
fn generate_workout_closure(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            expensive_closure(intensity)
        );
    }
}

//to fix that issue, we use a cacher, to *cache* the result of the closure
//Instead of saving the closure in a variable directly, we save a new instance of Cacher that holds the closure. 
//Then, in each place we want the result, we call the value method on the Cacher instance. 
//We can call the value method as many times as we want, or not call it at all, 
//and the expensive calculation will be run a maximum of once.
fn generate_workout_closure_cacher(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity > 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!",expensive_result.value(intensity));
    }
}
/* limitations of Cacher
 * Caching values is a generally useful behavior that we might want to use in other parts of our code with different closures. 
 * However, there are two problems with the current implementation of Cacher that would make reusing it in different contexts difficult.
 * 
 * 1:
 * a Cacher instance assumes it will always get the same value for the parameter `arg` to the value method, 
 * or in other words, the first time you call value on a cacher instance, it stores the result of the closure for the passes arguments, 
 * and returns the stored result, however, it doesn't redo the calculations when it's given a different value, meaning it will always return
 * the result of the first value call
 * 
 * this can be sorta fixed by having the Cacher hold a hashmap rather than a single value, 
 * the keys of the hashmap represent the arg values that are passed in, 
 * the values of the hashmap represent the result of calling the closure on that key
 * this way, instead of looking at whether self.value directly has a Some or None value, 
 * the value function will look up the arg in the hashmap and return the value if its present, 
 * and if its not present, call the closure and save the resulting value in the hash map associated with its arg value   
 * 
 * 2:
 * the current Cacher implementation only accepts closures that take one parameter of type u32 and return a u32. 
 * We might want to cache the results of closures that take a string slice and return usize values, for example. 
 * To fix this issue, try introducing more generic parameters to increase the flexibility of the Cacher functionality.
 * 
 */





///////////////////////////////////////////////////////////////
//Storing Closures Using Generic Parameters and the Fn Traits//
///////////////////////////////////////////////////////////////
/*
 * we can create a struct that will hold the closure and the resulting value of calling the closure. 
 * The struct will execute the closure only if we need the resulting value, 
 * and it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving and reusing the result. 
 * You may know this pattern as memoization or lazy evaluation.
 * 
 * To make a struct that holds a closure, we need to specify the type of the closure, 
 * because a struct definition needs to know the types of each of its fields. 
 * Each closure instance has its own unique anonymous type: that is, even if two closures have the same signature, 
 * their types are still considered different. 
 * 
 * To define structs, enums, or function parameters that use closures, we use generics and trait bounds, 
 * 
 * The Fn traits are provided by the standard library. All closures implement at least one of the traits: Fn, FnMut, or FnOnce
     * Fn:      borrows values from the environment immutably.
     * FnMut:   can change the environment because it mutably borrows values.
     * FnOnce:  consumes the variables it captures from its enclosing scope, known as the closure’s environment.
                To consume the captured variables, 
                the closure must take ownership of these variables and move them into the closure when it is defined. 
                The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, 
                so it can be called only once.
 * 
 * We add types to the Fn trait bound to represent the types of the parameters and return values 
 * the closures must have to match this trait bound. 
 * In this case, our closure has a parameter of type u32 and returns a u32, so the trait bound we specify is Fn(u32) -> u32.
*/
struct Cacher<T> 
where
    T: Fn(u32) -> u32,
{
    calculation: T, //calculation field of the generic type T. 
                    //The trait bounds on T specify that it’s a closure by using the Fn trait.
                    //Any closure we want to store in the calculation field must have one u32 parameter 
                    //(specified within the parentheses after Fn) 
                    //and must return a u32 (specified after the ->).
    value: Option<u32>, //The value field is of type Option<u32>. Before we execute the closure, value will be None. 
                        //When code using a Cacher asks for the result of the closure, 
                        //the Cacher will execute the closure at that time and store the result within a Some variant in the value field. 
                        //Then if the code asks for the result of the closure again, 
                        //instead of executing the closure again, the Cacher will return the result held in the Some variant.
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


//here, i'll redo the implementation of cacher to address the two issues
use std::collections::HashMap;
struct Cacher2<C,A,R> //these extra generic types help this implementation fix issue 2
where
    C: Fn(A) -> R,
{
    calculation: C,
    value: HashMap<A,R>
}
impl<C,A,R> Cacher2<C,A,R>
where
    C: Fn(A) -> R,
    A: std::cmp::Eq + std::hash::Hash + Copy,
    R: Copy,
{
    fn new(calculation: C) -> Cacher2<C,A,R> {
        Cacher2{
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: A) -> R { //this version of value fixes issue 1
        match self.value.get(&arg) {
            Some(&v) => v, //if a value exists in the hashmap for the key 'arg', return it
            None => { //otherwise, add it to the hashmap
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}