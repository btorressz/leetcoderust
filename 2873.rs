//2873. Maximum Value of an Ordered Triplet I

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut res: i64 = 0;
        let mut max_diff: i32 = 0;
        let mut max_val: i32 = 0;

        for &num in &nums {
            res = res.max(max_diff as i64 * num as i64);
            max_diff = max_diff.max(max_val - num);
            
            max_val = max_val.max(num);
        }

        res
    }
}
