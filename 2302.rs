//2302. Count Subarrays With Score Less Than K

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut w_sum: i64 = 0;
        let mut count: i64 = 0;
        let n = nums.len();
        let mut l: usize = 0;
        let mut r: usize = 0;

        while l < n {
            while r < n && (w_sum + nums[r] as i64) * ((r - l + 1) as i64) < k {
                w_sum += nums[r] as i64;
                r += 1;
            }

            count += (r - l) as i64;

            if l == r {
                r += 1;
            } else {
                w_sum -= nums[l] as i64;
            }

            l += 1;
        }

        count
    }
}
