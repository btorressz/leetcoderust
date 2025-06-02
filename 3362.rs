//3362. Zero Array Transformation III

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_removal(mut nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut used_query = BinaryHeap::new(); 
        let mut available_query = BinaryHeap::new(); 

        queries.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut query_pos = 0;
        let mut used_query_count = 0;

        for i in 0..n {
            while query_pos < queries.len() && queries[query_pos][0] == i as i32 {
                available_query.push(queries[query_pos][1]);
                query_pos += 1;
            }

            nums[i] -= used_query.len() as i32;

            while nums[i] > 0 {
                if let Some(&end) = available_query.peek() {
                    if end >= i as i32 {
                        available_query.pop();
                        used_query.push(Reverse(end)); 
                        nums[i] -= 1;
                        used_query_count += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            if nums[i] > 0 {
                return -1;
            }

            while let Some(&Reverse(end)) = used_query.peek() {
                if end == i as i32 {
                    used_query.pop();
                } else {
                    break;
                }
            }
        }

        (queries.len() - used_query_count) as i32
    }
}
