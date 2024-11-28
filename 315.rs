//315. Count of Smaller Numbers After Self
// Successful Attempt
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let offset = 10_000; 
        let size = 20_001; 
        let mut bit = vec![0; size]; 
        let mut result = vec![0; nums.len()];

        fn update(bit: &mut Vec<i32>, index: usize, value: i32) {
            let mut i = index;
            while i < bit.len() {
                bit[i] += value;
                i += i & (!i + 1);
            }
        }

        fn query(bit: &Vec<i32>, index: usize) -> i32 {
            let mut sum = 0;
            let mut i = index;
            while i > 0 {
                sum += bit[i];
                i -= i & (!i + 1);
            }
            sum
        }

        for (i, &num) in nums.iter().enumerate().rev() {
            let rank = (num + offset) as usize;
            result[i] = query(&bit, rank); 
            update(&mut bit, rank + 1, 1); 
        }

        result
    }
} //end of successful solution 
/*

//failed solution ONE WRONG ANSWER 

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let offset = 10_000; 
        let size = 20_001; 
        let mut seg_tree = vec![0; 4 * size]; 
        let mut result = vec![0; nums.len()];

        fn update(tree: &mut Vec<i32>, node: usize, start: usize, end: usize, index: usize) {
            if start == end {
                tree[node] += 1; 
                return;
            }
            let mid = (start + end) / 2;
            if index <= mid {
                update(tree, 2 * node, start, mid, index);
            } else {
                update(tree, 2 * node + 1, mid + 1, end, index);
            }
            tree[node] = tree[2 * node] + tree[2 * node + 1]; 
        }

        fn query(tree: &Vec<i32>, node: usize, start: usize, end: usize, left: usize, right: usize) -> i32 {
            if start > right || end < left {
                return 0; 
            }
            if start >= left && end <= right {
                return tree[node]; 
            }
            let mid = (start + end) / 2;
            let left_sum = query(tree, 2 * node, start, mid, left, right);
            let right_sum = query(tree, 2 * node + 1, mid + 1, end, left, right);
            left_sum + right_sum
        }

        for (i, &num) in nums.iter().enumerate().rev() {
            let rank = (num + offset) as usize;
            result[i] = query(&seg_tree, 1, 0, size - 1, 0, rank - 1); 
            update(&mut seg_tree, 1, 0, size - 1, rank); 
        }

        result
    } //end of failed solution 1
}*/

/* 
//FAILED ATTEMPT 2
use std::collections::BinaryHeap;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted = vec![]; 
        let mut result = vec![0; nums.len()];

        for (i, &num) in nums.iter().enumerate().rev() {
            let pos = sorted.binary_search(&num).unwrap_or_else(|x| x);
            result[i] = pos as i32; 
            sorted.insert(pos, num); 
        }

        result
    }
} // end of failed solution 2
*/
