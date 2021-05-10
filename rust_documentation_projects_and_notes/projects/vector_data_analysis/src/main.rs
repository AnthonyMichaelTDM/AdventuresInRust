use rand::Rng;
use std::collections::HashMap;
mod stair_sort;

fn main() {
    //generate a vector filled with random integers
    let mut v: Vec<usize> = Vec::new();
    let rand_length = rand::thread_rng().gen_range(50..=100);

    for _i in 0..=rand_length {
        let rand_value = rand::thread_rng().gen_range(0..=100);
        v.push(rand_value);
    }
    println!("unsorted array:");
    println!("{:?}", v);

    //find the average value of the vector
    let ave = average(&v);
    println!("average value of the vector: {}", ave);
    //sort the vector and find the median (middle value)
    let v = stair_sort::run(v, 5);
    println!("sorted array");
    println!("{:?}", v);

    println!("median value: {:?}", v.get(v.len()/2));

    //find the mode (most occurring) value with a hashmap
    let med = mode(&v);
    println!("mode value: {}", med);

}

fn average(v: &Vec<usize>) -> f32 {
    let mut total: usize = 0;
    let length = v.len();
    for e in v {
        total += e;
        //length += 1;
    }
    let ave: f32 = (total / length) as f32;
    return ave;
}

fn mode(v: &Vec<usize>) -> usize {
    let mut data_map = HashMap::new();
    for item in v.iter() {
        let i = data_map.entry(item).or_insert(0);
        *i += 1;
    }

    //find the max score
    let mut max: (&usize, usize) = (&0,0);
    for kv_pair in data_map {
        if kv_pair.1 > max.1 {
            max = kv_pair;
        }
    }
    return *max.0;
}
