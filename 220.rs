
/*
ATTEMPT ONE: TIME LIMIT EXCCEDED
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
