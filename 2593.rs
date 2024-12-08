//2593. Find Score of an Array After Marking All Elements

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_score(mut nums: Vec<i32>) -> i64 {
        let n = nums.len();
        
        // Create a min-heap 
        let mut min_heap = BinaryHeap::new();
        
        for (i, &value) in nums.iter().enumerate() {
            min_heap.push(Reverse((value, i)));
        }

        let mut score = 0;

        // Process the heap
        while let Some(Reverse((value, index))) = min_heap.pop() {
            // If the value at index `index` is not marked (-1), process it
            if nums[index] != -1 {
                score += value as i64;  // Add the value to the score
                nums[index] = -1;  // Mark the current index as processed
                // Mark adjacent indices as processed if within bounds
                if index > 0 {
                    nums[index - 1] = -1;  // Mark the previous index as processed
                }
                if index < n - 1 {
                    nums[index + 1] = -1;  // Mark the next index as processed
                }
            }
        }
        
        score
    }
}

