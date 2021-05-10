fn main() {
    //create a new string
    let _s = String::new();
    
    let _s = String::from("initial contents");
    
    
    let data = "initial contents";

    let _s = data.to_string();

    // the method also works on a literal directly:
    let _s = "initial contents".to_string();

    //Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them
    let mut vs: Vec<String> = Vec::new();
    vs.push(String::from("السلام عليكم"));
    vs.push(String::from("Dobrý den"));
    vs.push(String::from("Hello"));
    vs.push(String::from("שָׁלוֹם"));
    vs.push(String::from("नमस्ते"));
    vs.push(String::from("こんにちは"));
    vs.push(String::from("안녕하세요"));
    vs.push(String::from("你好"));
    vs.push(String::from("Olá"));
    vs.push(String::from("Здравствуйте"));
    vs.push(String::from("Hola"));
    for token in vs {
        println!("{}", token);
    }


    //Updating a string
    //you can change a string with .push/.pop, the + operator, and even the format! macro
    let mut s1 = String::from("foo");
    let s2 = "ba";
    s1.push_str(s2);    //push_str adds a string
    s1.push('r');       //push adds a character
    println!("s2 is {}", s2);

    //concatenation with the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is s1+s2 is {}", s3);
    //If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    //concatenation with the format! macro 
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);//the format macro is like a println that doesn't print, also, it doesn't take ownership of any of its parameters
    println!("{}", s);


    //internal representation
    let hello = String::from("Hola");
    println!("{} is {} bytes long", hello, hello.len());
    let hello = String::from("Здравствуйте"); //each of these characters is 2 bytes long
    println!("{} is {} bytes long despite only being {} characters", hello, hello.len(), hello.len() / 2); 

    //If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    //That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode scalar values, 
    //which are what Rust’s char type is, those bytes look like this:
    // ['न', 'म', 'स', '्', 'त', 'े']
    //There are six char values here, but the fourth and sixth are not letters: they’re diacritics that don’t make sense on their own. 
    //Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:
    // ["न", "म", "स्", "ते"]


    //slicing strings
    /*Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: 
    a byte value, a character, a grapheme cluster, or a string slice.*/

    //rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}",s);
    //if we were to slice &hello[0..1], rust would panic at run time bc that range only includes the first byte of hello, but every char
    //in hello is 2 bytes long

    //You should use ranges to create string slices with caution, because doing so can crash your program.



    //properly iterating over strings
    //iterate over char, aka scalar unicode values, in a string
    for token in "नमस्ते".chars() {
        println!("{}", token);
    }
    //iterate over the bytes in a string
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    //iterating over grapheme clusters is complex, therefore it's not provided in the standard library, 
    //there are crates on crates.io if this functionality is needed.

    //therefore, a safe way to slice characters from a string is as follows
    fn slice(input: &str, start_index: u32, end_index: u32) -> String {
        let mut i = 0;
        let mut output = String::new();
        for token in input.chars() {
            if i >= start_index && i < end_index { output.push(token); } else if i >= end_index { break }
            i += 1;
        }
        return format!("{}", output);
    }
    let s = slice(hello, 1, 4);
    println!("{}", s);



}
