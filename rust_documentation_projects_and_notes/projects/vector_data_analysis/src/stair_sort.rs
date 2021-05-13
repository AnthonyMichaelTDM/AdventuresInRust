use std::collections::HashMap;
use std::time::SystemTime;

struct VectorInfo {
    base_vector: Vec<usize>,
    sorted_vector: Vec<usize>,
    //length: usize,
    min: usize,
    max: usize,
}
impl VectorInfo {
    //constructor
    fn init(v: Vec<usize>) -> VectorInfo {
        VectorInfo {
            base_vector: v.clone(),
            sorted_vector: Vec::new(),
            //length: v.len(),
            min: v[0],
            max: v[0],
        }
    }
    //calculate, update, and return, the min and max
    fn min_max(&mut self) -> (usize, usize) {
        for i in &self.base_vector {
            if i > &self.max {
                self.max = *i;
            } else if i < &self.min {
                self.min = *i;
            }
        }
        (self.min, self.max)
    }
    //generate scaled array
    // fn gen_scaled(&mut self) {
    //     for i in 0..self.length {
    //         self.scaled_vector[i] -= self.min;
    //     }
    // }
}

struct Bins {
    _bin_count: usize,
    _bin_maxes: Vec<usize>,
    bins: Vec<Vec<usize>>,
}
impl Bins {
    //constructor
    fn init(bin_count: usize, v: &mut VectorInfo) -> Bins {
        let mut maxes: Vec<usize> = Vec::new();
        for i in 0..=(bin_count-1) {
            maxes.push( (v.max * i) / bin_count);
        }
        Bins {
            _bin_count: bin_count,
            _bin_maxes: maxes.clone(),
            bins: {
                let mut stair_bins: Vec<Vec<usize>> = Vec::new();
                for _i in 0..bin_count {
                    stair_bins.push(Vec::new());
                }
                for i in v.base_vector.iter()/*.rev()*/ {
                    let mut best_bin: usize = 0;
                    for e in 0..maxes.len() {
                        if *i > maxes[e] {
                            best_bin = e;
                        }
                    }
                    stair_bins[best_bin].push(*i/*v.base_vector.pop()*/);
                }
                v.base_vector.clear();
                stair_bins
            },
        }
    }

    //optomized count sort using hashmaps to count
    fn sort_bins_optimized(&mut self, v: &mut VectorInfo) {
        //go through the bins
        for bin in &mut self.bins {
            //find the min and max values of the bin, and count
            let mut bin_start_value: usize = 0;
            let mut bin_end_value: usize = 0;
            let mut bin_map = HashMap::new();
            for i in bin.iter() {
                //min and max
                if *i > bin_end_value {
                    bin_end_value = *i;
                } else if *i < bin_start_value {
                    bin_start_value = *i;
                }
                //build hashmap
                *bin_map.entry(i).or_insert(0) += 1;//count occurances
            }

            //build sorted vector
            //averages out to O(2n)
            for i in bin_start_value..=bin_end_value {
                let mut value = match bin_map.get(&i) {
                    Some(x) => *x,
                    _ => 0,
                };
                while 0<value {
                    //tmp_arr.push(i);
                    v.sorted_vector.push(i);//+v.min);
                    value -=1;
                }
            }
            // trouble shooting stuff
            //println!("bin: {:?}", bin);
            //println!("bin hash map: {:?}", bin_map);
            //println!(/*"sorted array so far: */"{:?}", v.sorted_vector);
            //println!("___");
            (*bin).clear();      
        }
    }
    /*
    fn sort_bins(&mut self, v: &mut VectorInfo) {
        //go through the bins
        for bin in &mut self.bins {
            //find the min and max values of the bin
            let mut bin_start_value: usize = 0;
            let mut bin_end_value: usize = 0;
            for i in bin.iter() {
                let bin_val = *i;
                if bin_val > bin_end_value {
                    bin_end_value = bin_val;
                } else if bin_val < bin_start_value {
                    bin_start_value = bin_val;
                }
            }
            let range: usize = bin_end_value - bin_start_value + 1;

            //2 auxillary vectors
            let mut count_arr: Vec<usize> = Vec::new();
            let mut tmp_arr: Vec<usize>   = Vec::new();
            //populate the auxillary vectors
            for _i in 0..range {count_arr.push(0)};
            for _i in 0..bin.len() {tmp_arr.push(0)};

            //count ocurances of each number in the arr
            for i in bin.iter() {count_arr[i - bin_start_value] += 1;}
            for i in 1..count_arr.len() {count_arr[i] += count_arr[i-1];}

            //build tmp_arr with count_arr then apply it to the bin
            for i in bin.iter().rev() {
                let bin_val = *i;
                tmp_arr[count_arr[bin_val - bin_start_value] -1] = bin_val;
                count_arr[bin_val - bin_start_value] -= 1;
            }

            //build sorted array
            for i in &tmp_arr {v.sorted_vector.push(i + v.min);}
            bin.clear();
            count_arr.clear();
            tmp_arr.clear();
        }
    }
    */
}
pub fn run(v: Vec<usize>, mut stair: usize) -> Vec<usize> {
    if stair <= 1 {
        stair = 1;
    }    
    let mut vector = VectorInfo::init(v.clone());

    let sys_time_before_sort = SystemTime::now();
    //find the min and max values
    vector.min_max();

    let sys_time_after_sort = SystemTime::now();
    let difference = sys_time_after_sort.duration_since(sys_time_before_sort).expect("time may have gone backwards");
    println!("mins and max's found in {} seconds!", difference.as_secs_f32());
    
    //scale the array such that the min is zero
    //vector.gen_scaled();
    //vector.max -= vector.min;

    let sys_time_before_sort = SystemTime::now();
    //generate the bins
    let mut bins = Bins::init(stair, &mut vector);

    let sys_time_after_sort = SystemTime::now();
    let difference = sys_time_after_sort.duration_since(sys_time_before_sort).expect("time may have gone backwards");
    println!("bins generated in {} seconds!", difference.as_secs_f32());
    let sys_time_before_sort = SystemTime::now();
    //sort the bins, also generates the sorted array
    bins.sort_bins_optimized(&mut vector);
    
    let sys_time_after_sort = SystemTime::now();
    let difference = sys_time_after_sort.duration_since(sys_time_before_sort).expect("time may have gone backwards");
    println!("bins sorted in {} seconds!", difference.as_secs_f32());

    vector.sorted_vector.clone()
}
