use std::cmp::{Ordering, min, max};
use std::collections::BinaryHeap;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let k = nums.len();
        let mut heap = BinaryHeap::new();
        let mut max_val = i32::MIN;
        let mut range = vec![0, i32::MAX];

        for i in 0..k {
            if let Some(&num) = nums[i].first() {
                heap.push((std::cmp::Reverse(num), i, 0));
                max_val = max(max_val, num);
            }
        }

        while let Some((std::cmp::Reverse(min_val), list_idx, elem_idx)) = heap.pop() {
            if max_val - min_val < range[1] - range[0] {
                range = vec![min_val, max_val];
            }
            if elem_idx + 1 < nums[list_idx].len() {
                let next_val = nums[list_idx][elem_idx + 1];
                heap.push((std::cmp::Reverse(next_val), list_idx, elem_idx + 1));
                max_val = max(max_val, next_val);
            } else {
                break;
            }
        }

        range
    }
}
