fn main() {
    //A string slice is a reference to part of a String, and it looks like this:
    let s = String::from("hello world");
    let _hello = &s[0..5]; //"hello "
    let _world = &s[6..11];//"world"

    //With Rust’s .. range syntax, if you want to start at the first index (zero), 
    //you can drop the value before the two periods. In other words, these are equal:
    let _slice_from_zero = &s[0..2];
    let _slice_from_zero = &s[..2];

    //By the same token, if your slice includes the last byte of the String, 
    //you can drop the trailing number. That means these are equal:
    let _slice_to_end = &s[3..s.len()];
    let _slice_to_end = &s[3..];

    //You can also drop both values to take a slice of the entire string. So these are equal:
    let _slice_of_whole = &s[0..s.len()];
    let _slice_of_whole = &s[..];


    let mut s = String::from("hello world");

    let word = first_word(&s);

    //s.clear(); // error!

    println!("the first word is: {}", word);



    //string literals are slices 
    let s1 = "hello world"; //type: &str //immutable //stored on the stack
    let s2 = String::from("hello world"); //type: string //mutable //stored in the heap

    

    let my_string = String::from("hello world");
    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);


    //you can also take slices of arrays
}

//finds the first work of a given string literal, returns that word as a slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

