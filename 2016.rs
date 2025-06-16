//2016. Maximum Difference Between Increasing Elements

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut md = i32::MAX;
        let mut res = -1;

        for &x in &nums {
            if x > md {
                res = res.max(x - md);
            } else {
                md = x;
            }
        }

        res
    }
}
