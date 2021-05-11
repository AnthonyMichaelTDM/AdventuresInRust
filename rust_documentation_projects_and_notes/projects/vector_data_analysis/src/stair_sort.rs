struct VectorInfo {
    base_vector: Vec<usize>,
    scaled_vector: Vec<usize>,
    sorted_vector: Vec<usize>,
    length: usize,
    min: usize,
    max: usize,
}
impl VectorInfo {
    //constructor
    fn init(v: Vec<usize>) -> VectorInfo {
        VectorInfo {
            base_vector: v.clone(),
            scaled_vector: v.clone(),
            sorted_vector: v.clone(),
            length: v.len(),
            min: *&v[0],
            max: *&v[0],
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
        self.base_vector.clear();
        return (self.min, self.max);
    }
    //generate scaled array
    fn gen_scaled(&mut self) {
        for i in 0..self.length {
            self.scaled_vector[i] -= self.min;
        }
    }
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
                for i in v.scaled_vector.iter() {
                    let mut best_bin: usize = 0;
                    for e in 0..maxes.len() {
                        if *i > maxes[e] {
                            best_bin = e;
                        }
                    }
                    stair_bins[best_bin].push(*i);
                }
                v.scaled_vector.clear();
                v.base_vector.clear();
                v.sorted_vector.clear();
                stair_bins
            },
        }
    }

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
            for i in 0..tmp_arr.len() {v.sorted_vector.push(tmp_arr[i] + v.min);}
            bin.clear();
            count_arr.clear();
            tmp_arr.clear();
        }
    }
}
pub fn run(v: Vec<usize>, mut stair: usize) -> Vec<usize> {
    if stair <= 2 {
        stair = 2;
    }

    let mut vector = VectorInfo::init(v);
    //find the min and max values
    vector.min_max();

    //scale the array such that the min is zero
    vector.gen_scaled();
    vector.max -= vector.min;

    //generate the bins
    let mut bins = Bins::init(stair, &mut vector);

    //sort the bins
    bins.sort_bins(&mut vector);
    
    return vector.sorted_vector.clone();
}
