//2657. Find the Prefix Common Array of Two Arrays

use std::collections::HashMap;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut cntA = HashMap::new();
        let mut cntB = HashMap::new();

        for (&ai, &bi) in a.iter().zip(b.iter()) {
            *cntA.entry(ai).or_insert(0) += 1;
            *cntB.entry(bi).or_insert(0) += 1;
            
            let t: i32 = cntA.iter()
                             .map(|(&x, &v)| v.min(*cntB.get(&x).unwrap_or(&0)))
                             .sum();
                             
            result.push(t);
        }

        result
    }
}
