use rand::Rng;
use std::collections::HashMap;
use std::time::SystemTime;
mod stair_sort;

fn main() {
    //generate a vector filled with random integers
    let mut v: Vec<usize> = Vec::new();
    let rand_length = 100_000_000;//rand::thread_rng().gen_range(1_000_000..=10_000_000);
    
    let sys_time_before_sort = SystemTime::now();
    for _i in 0..=rand_length {
        let rand_value = rand::thread_rng().gen_range(0..=100_000_000);
        v.push(rand_value);
    }
    let sys_time_after_sort = SystemTime::now();
    let difference = sys_time_after_sort.duration_since(sys_time_before_sort).expect("time may have gone backwards");
    
    println!("generated unsorted array in {} seconds!", difference.as_secs_f32());
    //println!("{:?}", v);

    //find the average value of the vector
    let ave = average(&v);
    println!("average value of the vector: {}", ave);
    //sort the vector and find the median (middle value)
    let sys_time_before_sort = SystemTime::now();
    let v = stair_sort::run(v, 100);
    let sys_time_after_sort = SystemTime::now();
    let difference = sys_time_after_sort.duration_since(sys_time_before_sort).expect("time may have gone backwards");
    println!("sorted array in {} seconds", difference.as_secs_f32());
    //println!("{:?}", v);

    //verify the sort
    let mut is_sorted = true;
    for i in 1..(v.len()-1) {
        let prev_value = &v[i];
        let curr_value = &v[i];
        let next_value = &v[i];
        if !(curr_value >= prev_value && curr_value <= next_value) {is_sorted = false;}
    }
    println!("sorted successfully? {}", is_sorted);

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
