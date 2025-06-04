//2962. Count Subarrays Where Max Element Appears at Least K Times
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut count = 0;
        let mut res = 0;
        let mut l = 0;
        let mut r = 0;
        let n = nums.len();
        let max = *nums.iter().max().unwrap();

        while r < n {
            if nums[r] == max {
                count += 1;
            }

            while l <= r && count >= k {
                if nums[l] == max {
                    count -= 1;
                }
                res += (n - r) as i64;
                l += 1;
            }

            r += 1;
        }

        res
    }
}
