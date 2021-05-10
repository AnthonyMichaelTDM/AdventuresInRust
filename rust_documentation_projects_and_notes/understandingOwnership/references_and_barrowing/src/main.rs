fn main() {
    //example that takes, and then gives ownership, but is also able to do other stuff, using tuples
    let s1 = String::from("she sells sea shells at the sea shore");
    let (s1, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s1, len);
    //this is tedious though, luckily rust has something called references

    //& is the reference operator
    //* is the dereference operator
    //both of these are used in function parameters

    let _len = calculate_length_reference(&s1);

    //you *usually* can't modify references, but there is a way to be able to do it, &mut
    let mut s2 = s1.clone();
    change(&mut s2);
    println!("{}",s2);

    //one limitation is that there can only be one mutable refernce to a particular piece of data in a particular scope
    //also cannot have any immutable references when you have a mutable one
    /* for example, this doesn't work
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);*/
    //but this does
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    /*In languages with pointers, it’s easy to erroneously create a dangling pointer, 
    a pointer that references a location in memory that may have been given to someone else, 
    by freeing some memory while preserving a pointer to that memory. 
    In Rust, by contrast, the compiler guarantees that references will never be dangling references: 
        if you have a reference to some data, the compiler will ensure that the data will not go out of scope 
        before the reference to the data does.
    
    to prevent this, just don't have functions that return references
    */


}

//using a tuple to return ownership
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_reference(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", but the value of those shells will fall"); //.push_str appends to the string
}
