//857. Minimum Cost to Hire K Workers

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut t: Vec<(f64, i32)> = quality
            .into_iter()
            .zip(wage.into_iter())
            .map(|(q, w)| (w as f64 / q as f64, q))
            .collect();
        
        // Sort by the ratio of wage/quality
        t.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        
        let mut tot = 0;
        let mut ans = f64::INFINITY;
        let mut pq: BinaryHeap<i32> = BinaryHeap::new();  // max-heap to store qualities

        for (x, q) in t {
            tot += q;
            pq.push(q);
            
            if pq.len() == k as usize {
                ans = ans.min(x * tot as f64);
                tot -= pq.pop().unwrap(); // Pop the largest quality from the heap
            }
        }
        
        ans
    }
}
