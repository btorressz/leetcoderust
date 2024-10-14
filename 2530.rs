use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
        
        for num in nums {
            max_heap.push(num);
        }

        let mut score: i64 = 0;
        let k = k as usize;

        for _ in 0..k {
            if let Some(max_elem) = max_heap.pop() {
                score += max_elem as i64; 

                let new_elem = (max_elem + 2) / 3; 
                if new_elem > 0 {
                    max_heap.push(new_elem);
                }
            }
        }

        score
    }
}
