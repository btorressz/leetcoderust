//SUCCESSFUL ATTEMPT 
//220. Contains Duplicate III

use std::collections::BTreeSet;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        if value_diff < 0 {
            return false; 
        }

        let mut btree = BTreeSet::new();
        let index_diff = index_diff as usize;

        for (i, &num) in nums.iter().enumerate() {
            let current = num as i64;

            if btree
                .range((current - value_diff as i64)..) 
                .next()                               
                .filter(|&&lower_bound| lower_bound <= current + value_diff as i64) 
                .is_some()
            {
                return true;
            }

            btree.insert(current);

            if i >= index_diff {
                btree.remove(&(nums[i - index_diff] as i64));
            }
        }

        false
    }
}

/*
//ATTEMPT ONE: TIME LIMIT EXCCEDED
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let n = nums.len();

        for i in 0..n {
            for j in i + 1..n {
                if (j - i) as i32 > index_diff {
                    break;
                }
                if (nums[i] as i64 - nums[j] as i64).abs() <= value_diff as i64 {
                    return true;
                }
            }
        }

        false
    }
}*/
