//260. Single Number III
//Solution 1: Sorting 
impl Solution {
    pub fn single_number(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut result = Vec::new();
        let n = nums.len();
        let mut i = 0;
        while i < n {
            if i + 1 < n && nums[i] == nums[i + 1] {
                i += 2; 
            } else {
                result.push(nums[i]); 
                i += 1;
            }
            if result.len() == 2 {
                break;
            }
        }
        result
    }
} // end of solution 1

//Solution 2: Hash Map
use std::collections::HashMap;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut freq_map = HashMap::new();
        for num in nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }
        freq_map.into_iter()
            .filter(|&(_, count)| count == 1)
            .map(|(num, _)| num)
            .collect()
    }
} // end of solution 2



