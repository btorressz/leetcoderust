//781. Rabbits in Forest

use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut freq = HashMap::new();

        for &res in &answers {
            *freq.entry(res).or_insert(0) += 1;
        }

        let mut tot = 0;

        for (&key, &count) in freq.iter() {
            let group_size = key + 1;
            let groups = (count + group_size - 1) / group_size; 
            tot += groups * group_size;
        }

        tot
    }
}
