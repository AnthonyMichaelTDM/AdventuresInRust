fn main() {
    /*
     * Processing a Series of Items with Iterators
     * 
     * The iterator pattern allows you to perform some task on a sequence of items in turn. 
     * An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. 
     * When you use iterators, you don’t have to reimplement that logic yourself.
     * 
     * In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up. 
     * For example, the code in Listing 13-10 creates an iterator over the items in the vector v1 by calling the iter method defined on Vec<T>. 
     * This code by itself doesn’t do anything useful.
     */
    let v1 = vec![1,2,3];

    let v1_iter = v1.iter();
    /*
     * The iterator is stored in the v1_iter variable, 
     * Once created, iterators can be used in a variety of ways.
     * 
     * when you write code in the patern
     * ```
     *  for value in v1 {
     *      //do something
     *  }
     * ```
     * you're implicitly creating and consuming an iterator over v1
     * 
     * here, we separate the creation of the ieterator from its use in the for loop
     * each element in the iterator is used in one iteration of the loop
     */
    for val in v1_iter {
        println!("Got: {}", val);
    }
    /*
     * in languaged with out iterators in their standard libraries, 
     * you'd likely write the same functionality by starting a variable at index 0,
     * using that variable to index into the vector to get a value,
     * and incrementing the variable value in a loop until it reached the total number of items in the vector
     * 
     * iterators handle all that logic for you, cutting down on repetitive code and boilerplate,
     * all while giving the flexibility to use that logic with all sorts of sequences, 
     * not just indexable data structures
     */
}

#[cfg(test)]
mod basic_tests {
    ////////////////////////////////////////////
    // The Iterator Trait and the next Method //
    ////////////////////////////////////////////
    /*
     * all iterators implement a trait named `Iterator` that is defined  in the standard library.
     * the definition of the trait looks like this:
     * 
     * ```
     *  fn main() {
     *  pub trait Iterator {
     *      type Item;
     *  
     *      fn next(&mut self) -> Option<Self::Item>;
     *  
     *      // methods with default implementations elided
     *  }
     * ```
     * 
     * this definition uses some new syntax: `type Item` and `Self::Item`,
     * which are defining an *associated type* with this trait.
     * Associated types will be discussed in depth in ch19, 
     * but what this code says is that implementing the `Iterator` trait requires
     * you also define an `Item` type, which is used in the return type of the `next` method.
     * 
     * The `Iterator` trait only requires implementors to define one method: 
     * the `next` method, which returns one item of the iterator at a time wrapped in `Some` and, when iteration is over, returns `None`.
     * 
     * We can call the `next` method on iterators directly, as shown below
     */
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
    /*
     * Note that we needed to make `v1_iter` mutable, this is because:
     * calling the `next` method on an iterator changes the internal state that the interator uses to keep track of where it is in the sequence.
     * 
     * This code consumes the iterator, each call to `next` eats up an item from the iterator
     * 
     * We didn't ned to make v1_iter mutable when we used a `for` loop because the loop took ownership of v1_iter and made it mutable behind the scenes
     * 
     * Also note that the values we get from the calls to `next` are immutable references to the values in the vector.
     * the `iter` method produces an iterator over immutable references. 
     *      If we want to create an iterator that takes ownership of `v1` and returns owned values, 
     *      we can call `into_iter` instead of `iter`. 
     *      
     *      Similarly, if we want to iterate over mutable references, 
     *      we can call `iter_mut` instead of `iter`.
     */
    
    ///////////////////////////////////////
    // Methods that Consume the Iterator //
    ///////////////////////////////////////
    /*
     * The `Iterator` trait has a number of different methods with default implementations provided by the standard library; 
     *      you can find out about these methods by looking in the standard library API documentation for the `Iterator` trait.
     * Some of these methods call the `next` method in their definition, 
     * which is why you’re required to implement the next method when implementing the `Iterator` trait
     * 
     * Methods that call `next` are called *consuming adaptors*, 
     * because calling them uses up the iterator.
     */
    #[test]
    fn iterator_sum() {
        /*
         * One example is the `sum` method, which takes ownership of the iterator and iterates through the items by repeatedly calling `next`,
         * thus consuming the iterator.
         */
        let v1 = vec![1,2,3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total,6);
        /*
         * We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.
         * so even though `v1_iter.next() would return `None`, we can't do that
         */
    }


    //////////////////////////////////////////
    // Methods that produce other iterators //
    //////////////////////////////////////////
    /*
     * *iterator adaptors* are methods defined on the `Iterator` trait that don't consume the iterator.
     * Instead, they produce different iterators by chaning some aspects of the original iterator.
     * 
     * the example below calls the iterator adaptor method `map`, 
     * which takes a closure to call on each item as they are iterated through, 
     * and returns a new iterator over the modifies items.
     */
    #[test]
    fn iterator_map() {
        let v1: Vec<i32> = vec![1,2,3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2,3,4]);
    }
    /* You can chain multiple aclls to iterator adaptors to perform complex actions in a readable way.
     * But, because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to the iterator adaptors. 
     */
}   

///////////////////////////////////////////////////
// Using closures that capture their environment //
///////////////////////////////////////////////////
/*
 * many iterator adaptors take closures as arguments, 
 * and commonly the closures we'll specify as arguments to iterator adaptors will be closures that capture their environment.
 * 
 * For this example, we'll use the `filter` method that takes a closure. The closure gets an item from the iterator and returns a `bool`.
 * If the closure returns `true`, the value will be included in the iterator produced by `filter`.
 * If the closure returns `false`, the value won't be included.
 * 
 * here, we use `filter` with a closure that captures the `shoe_size` variable from its environment to iterate over a collection of `Shoe` struct instances. It will return only shoes that are the specified size.
 */

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
/*
 * The `shoes_in_size` function takes ownership of a vector of shoes and a shoe size as parameters.
 * It returns a vector containing only shoes of the specified size.
 * 
 * In the body of `shoes_in_size`, we call `into_iter` to create an iterator that takes owndership of the vector. 
 * Then we call `filter` to adapt that iterator into a new iterator that only contains elements for which the closure returns `true`.
 * 
 * The closure captures the `shoe_size` parameter from the environment and compares the value with each shoe’s size, 
 * keeping only shoes of the size specified. 
 * Finally, calling `collect` gathers the values returned by the adapted iterator into a vector that’s returned by the function.
 */
#[allow(dead_code)]
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
/*
 * The test shows that when we call `shoes_in_size`, 
 * we get back only shoes that have the same size as the value we specified.
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}


