//396. Rotate Function

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let total_sum: i32 = nums.iter().sum();
        let mut res = 0;
        
        for (i, &num) in nums.iter().enumerate() {
            res += i as i32 * num;
        }
        
        let mut max_val = res;

        for i in 1..n {
            res = res + total_sum - (n as i32) * nums[n - i];
            max_val = max_val.max(res);
        }
        
        max_val
    }
}
