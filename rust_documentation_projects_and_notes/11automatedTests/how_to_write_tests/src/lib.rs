/*
 * Correctness in our programs is the extent to which our code does what we intend it to do.
 * Rust is designed with a high degree of concern about the correctness of programs, but correctness is complex and not easy to prove.
 * Rust’s type system shoulders a huge part of this burden, but the type system cannot catch every kind of incorrectness.
 * As such, Rust includes support for writing automated software tests within the language.
 *
 * Tests are Rust functions that verify that the non-test code is functioning in the expected manner.
 * The bodies of test functions typically perform these three actions:
 * Set up any needed data or state.
 * Run the code you want to test.
 * Assert the results are what you expect.
 * Let’s look at the features Rust provides specifically for writing tests that take these actions,
 * which include the test attribute, a few macros, and the should_panic attribute
*/

/*
 * At its simplest, a test in Rust is a function that’s annotated with the test attribute.
 * Attributes are metadata about pieces of Rust code; one example is the derive attribute we used with structs in Chapter 5.
 * To change a function into a test function, add #[test] on the line before fn. When you run your tests with the cargo test command,
 * Rust builds a test runner binary that runs the functions annotated with the test attribute and reports
 * on whether each test function passes or fails.
*/

#[cfg(test)]
mod tests {

    use super::*; //allow the tests to use other methods in the library

    //Note the #[test] annotation before the fn line: this attribute indicates this is a test function,
    //so the test runner knows to treat this function as a test.
    //We could also have non-test functions in the tests module to help set up common scenarios or perform common operations,
    //so we need to indicate which functions are tests by using the #[test] attribute.
    
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }
    
    
    
    /* running this test will return something like this:
     * running 1 test
     * test tests::it_works ... ok
     * test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     *
     *    Doc-tests adder
     * running 0 tests
     * test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     *
     *
     * the first tests are the ones we added here, the second bit is for doc tests,
     * We don’t have any documentation tests yet, but Rust can compile any code examples that appear in our API documentation.
     * This feature helps us keep our docs and our code in sync
     */

    /*
     * Let’s add another test, but this time we’ll make a test that fails!
     * Tests fail when something in the test function panics.
     * Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.
     */
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    /* Checking Results with the assert! Macro
     *
     * The assert! macro, provided by the standard library,
     * is useful when you want to ensure that some condition in a test evaluates to true.
     * We give the assert! macro an argument that evaluates to a Boolean.
     * If the value is true, assert! does nothing and the test passes.
     * If the value is false, the assert! macro calls the panic! macro, which causes the test to fail
     *
     * The can_hold method returns a Boolean, which means it’s a perfect use case for the assert!
     *
     */
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger._can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller._can_hold(&larger)); //because the can_hold method should return false, we need to negate it for the test towork sucessfully
    }

    // testing equality with assert_eq! and assert_ne!
    /*
     * A common way to test functionality is to compare the result of the code under test to
     * the value you expect the code to return to make sure they’re equal.
     *
     * You could do this using the assert! macro and passing it an expression using the == operator.
     * However, this is such a common test that the standard library provides a pair of macros
     * assert_eq!
     *  assert_ne!
     * to perform this test more conveniently. These macros compare two arguments for equality or inequality, respectively.
     * They’ll also print the two values if the assertion fails, which makes it easier to see why the test failed;
     *
     * conversely, the assert! macro only indicates that it got a false value for the == expression,
     * not the values that lead to the false value.
     *
     *
     */
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn it_sorts() {
        let v: Vec<usize> = vec!(1,4,6,2,7,2,8,9,0);
        assert_eq!(vec!(0,1,2,2,4,6,7,8,9), _count_sort(&v));
    }
    #[test]
    fn does_it_add_two() {
        assert_eq!(4, bad_add_two(2));
    }
    //this last one fails, but notice how it gives useful debugging information




    //adding custom failure messages
    /*
     * sometimes, the info returned by the assert macros isn't enough for proper diagnosis, 
     * in these cases you can add a custom error message to give more debugging information
     * 
     * 
     * 
    */
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
    //This result just indicates that the assertion failed and which line the assertion is on. 
    //A more useful failure message in this case would print the value we got from the greeting function. 

    //Let’s change the test function, giving it a custom failure message made from a format string with a placeholder filled in with the actual value we got from the greeting function:
    #[test]
    fn greeting_contains_name_custom_message() {
        let result = greeting("Stacy");
        assert!(
            result.contains("Carol"), //the boolean to assert
            "Greeting did not contain name, value was {}", //format string
            result
        );
    }



    //testing for panics with #[should_panic]
    /*
     * the should_panic attribute makes it so that a test passes if it panics,
     * this is usefull to test your functions for error handling, or testing edge cases
     * 
     * by default, should panic is limited bc it only will tell you that the code paniced, but that panic may not be what you expecte,
     * you can make should_panic more precise by adding the optional expects parameter, which will make it so the test only passes if
     * the code panics, and the panic is for the reason you expected
    */

    //panics, passes
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    //doesn't panic, fails
    #[test]
    #[should_panic]
    fn less_than_100() {
        Guess::new(80);
    }

    //panics, for the expected reason, passes
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")] 
    //note that the expected only needs to be a slice of the actual panic message, not neccessarily the entire message
    fn greater_than_100_expected_passes() {
        Guess::new(200);
    }

    //panics, but not for the expected reason, fails
    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn greater_than_100_expected_fails() {
        Guess::new(200);
    }






    //using Result<T, E> in tests
    #[test]
    fn it_works_results() -> Result<(), String> {
        let x = 2+2;
        if x == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    /*
     * The it_works function now has a return type, Result<(), String>. 
     * In the body of the function, rather than calling the assert_eq! macro, 
     * we return Ok(()) when the test passes and an Err with a String inside when the test fails.
     * 
     * Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests, 
     * which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.
     * 
     * You can’t use the #[should_panic] annotation on tests that use Result<T, E>. 
     * Instead, you should return an Err value directly when the test should fail.
    */



    
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
pub fn bad_add_two(a: i32) -> i32 { a+3 }
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

use std::collections::HashMap;
pub fn _count_sort (v: &[usize]) -> Vec<usize> {
    let mut sorted_vector = Vec::new();
    
    //find the min and max values of the bin, and count
    let mut start_value: usize = 0;
    let mut end_value: usize = 0;
    let mut vec_map = HashMap::new();
    for i in v.iter() {
        //min and max
        if *i > end_value {
            end_value = *i;
        } else if *i < start_value {
            start_value = *i;
        }
        //build hashmap
        *vec_map.entry(i).or_insert(0) += 1; //count occurances
    }

    //build sorted vector
    for i in start_value..=end_value {
        let mut value = match vec_map.get(&i) {
            Some(x) => *x,
            _ => 0,
        };
        while 0 < value {
            sorted_vector.push(i);
            value -= 1;
        }
    }

    sorted_vector
}



pub struct Guess {
    _value: i32,
}
impl Guess {
    pub fn new(_value: i32) -> Guess {
        if _value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                _value
            );
        } else if _value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                _value
            );
        }

        Guess { _value }
    }
}