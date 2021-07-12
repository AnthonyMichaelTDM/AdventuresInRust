//packages and whatnot
use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;
use std::time::SystemTime;
use std::thread;

const MAX_SIZE: usize = 100_000_000; //max size of the array
const STACK_SIZE: usize = 4 * 1024 * 1024 * 1024;//4 gb
const RANGE:usize = 32767;

fn run() {
    // Usual main code goes here
    //DATA
    let mut arr:[usize; MAX_SIZE] = [0; MAX_SIZE];//array to sort later
    let mut rng = rand::thread_rng();

    //fill array with random numbers between 0 and 32767
    for i in 0..MAX_SIZE {
        arr[i] = Uniform::from(0..RANGE+1).sample(&mut rng);
    }

    //sort
    println!("sorting");
    count_sort_vanilla(arr);
} 

fn main() {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}//main

fn min_max(arr: &[usize; MAX_SIZE]) -> (usize, usize) {
    //DATA
    let mut min = arr[0];
    let mut max = arr[0];
    
    //find mins and maxes
    for i in arr.iter().enumerate() {
        if i.1 > &max {
            max = *i.1;
        } else if i.1 < &min {
            min = *i.1;
        }
    }

    println!("min {}, max {}",min,max);
    (min, max)
}//min_max

fn count_sort(mut arr: [usize; MAX_SIZE]) {
    //DATA
    let mut count_map = HashMap::new();
    let mut value; //value part of count hashmaps Key Value pair, used when rebuilding array
    let (min, max) = min_max(&arr);
    let sys_time_before_sort;
    let sys_time_after_sort;
    let mut j = 0; //used to index arr when rebuilding array

    //start time
    sys_time_before_sort = SystemTime::now();

    //build count hashmap
    for i in 0..MAX_SIZE {
        *count_map.entry(arr[i]).or_insert(0) += 1;//count occurances
    }

    //build sorted vector
    //averages out to O(2n)
    for i in min..=max {
        value = match count_map.get(&i) {
            Some(x) => *x,
            _ => 0,
        };
        while 0<value {
            arr[j] = i;//+v.min);
            value -=1;
            j += 1;
        }
    }

    //end time, and output results
    sys_time_after_sort = SystemTime::now();
    println!("sorted in {} seconds", sys_time_after_sort.duration_since(sys_time_before_sort).expect("time may have gone backwards").as_secs_f32())
}//count_sort

fn count_sort_vanilla(mut arr: [usize; MAX_SIZE]) {
    //DATA
    let (min, max) = min_max(&arr);
    let mut count_arr = [0; RANGE+1];//stores counts of each value, the index is the value, and the value of that index is the count ... kinda like a hashmap almost (in fact, this is pretty much the perfect use case for hashmaps)
    let mut tmp_arr = [0; MAX_SIZE];//used when building the sorted array before overwriting the base array
    let sys_time_before_sort;
    let sys_time_after_sort;
    
    //start timer
    sys_time_before_sort = SystemTime::now();

    //count occurances of each number in arr
    for i in 0..MAX_SIZE {
        count_arr[arr[i] - min] += 1;
    }

    //store cumulative count, helps for unpacking later
    for i in 1..RANGE {
        count_arr[i] += count_arr[i-1];
    }

    //build tmpArr with countArr
    for i in (0..MAX_SIZE).rev() {
        //optimization of what was previously several nested for loops, i made this optomization after a realization when playing computer, 
        //BASICALLY
        //restore value to tmp arr
        tmp_arr[count_arr[arr[i] - min ] - 1] = arr[i];
        //decrement the count of that value
        count_arr[arr[i] - min] -= 1;
    }

    //rebuild orginal array, but sorted now
    for i in 0..MAX_SIZE {
        arr[i] = tmp_arr[i];
    }

    //end time, and output results
    sys_time_after_sort = SystemTime::now();
    println!("sorted in {} seconds", sys_time_after_sort.duration_since(sys_time_before_sort).expect("time may have gone backwards").as_secs_f32())
}