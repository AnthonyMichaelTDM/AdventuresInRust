use std::collections::HashMap;

fn main() {
    //creating a new hash map
    
    //we’re keeping track of the scores of two teams whose names are Blue and Yellow. The Blue team starts with 10 points, and the Yellow team starts with 50.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
    //Just like vectors, hash maps store their data on the heap

    //Another way of constructing a hash map is by using iterators and the collect method on a vector of tuples, where each tuple consists of a key and its value.
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);
    /* The type annotation HashMap<_, _> is needed here because 
    it’s possible to collect into many different data structures and Rust doesn’t know which you want unless you specify. 
    For the parameters for the key and value types, however, we use underscores, 
    and Rust can infer the types that the hash map contains based on the types of the data in the vectors. */


    //Hash maps and ownership
    /* For types that implement the Copy trait, like i32, the values are copied into the hash map. 
    For owned values like String, the values will be moved and the hash map will be the owner of those values */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);// field_name and field_value are invalid at this point, try using them and see what compiler error you get!
    /* If we insert references to values into the hash map, the values won’t be moved into the hash map. 
    The values that the references point to must be valid for at least as long as the hash map is valid. */


    //accessing values in a hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Team {} has {:?} point(s)", team_name, score); //The result is wrapped in Some because get returns an Option<&V>
    //We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a for loop:
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    //This code will print each pair in an arbitrary order


    //updating a hash map
    /*Although the number of keys and values is growable, each key can only have one value associated with it at a time. 
    When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned. 
    You could replace the old value with the new value, 
    completely disregarding the old value. 
    You could keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value. 
    Or you could combine the old value and the new value. Let’s look at how to do each of these!*/

    //overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    //Only Inserting a Value If the Key Has No Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    //Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); //The or_insert method actually returns a mutable reference (&mut V) to the value for this key. 
        //Here we store that mutable reference in the count variable

        *count += 1; //in order to assign to that value, we must first dereference count using the asterisk (*).
    }
    println!("{:?}", map);



    /* By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables1. 
    This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. 
    If you profile your code and find that the default hash function is too slow for your purposes, 
    you can switch to another function by specifying a different hasher. A hasher is a type that implements the BuildHasher trait. */


    //summary of collections
    /* Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data. 
    Here are some exercises you should now be equipped to solve: */

    /*
    Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), 
    and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    */
    /*
    Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” 
    Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    */
    /*Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, 
    “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people 
    in the company by department, sorted alphabetically.
    */
}
