//3066. Minimum Operations to Exceed Threshold Value II

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        // Use a min-heap (BinaryHeap with Reverse to store values in ascending order)
        let mut min_heap: BinaryHeap<Reverse<i64>> = nums.into_iter().map(|x| Reverse(x as i64)).collect();
        let mut count = 0;

        while let Some(Reverse(min1)) = min_heap.pop() {
            // If the smallest element is already >= k, it's done
            if min1 >= k as i64 {
                break;
            }

            // Extract the second smallest element
            if let Some(Reverse(min2)) = min_heap.pop() {
                // Compute the new value using the formula: 2 * min + max
                let new_value = 2 * min1.min(min2) + min1.max(min2);
                // Push the new value back into the heap
                min_heap.push(Reverse(new_value));
                // Increment the operation count
                count += 1;
            }
        }

        count
    }
}

