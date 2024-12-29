//689. Maximum Sum of 3 Non-Overlapping Subarrays

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;  
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;
        let mut mx1 = 0;
        let mut mx12 = 0;
        let mut idx1 = 0;
        let mut idx12 = (0, 0);
        let mut max_sum = 0;  
        let mut ans = Vec::new();

        for i in k * 2..n {
            s1 += nums[i - k * 2];
            s2 += nums[i - k];
            s3 += nums[i];
            
            if i >= k * 3 - 1 {
                if s1 > mx1 {
                    mx1 = s1;
                    idx1 = i - k * 3 + 1;
                }
                if mx1 + s2 > mx12 {
                    mx12 = mx1 + s2;
                    idx12 = (idx1, i - k * 2 + 1);
                }
                if mx12 + s3 > max_sum {
                    max_sum = mx12 + s3;
                    ans = vec![idx12.0 as i32, idx12.1 as i32, (i - k + 1) as i32];
                }
                s1 -= nums[i - k * 3 + 1];
                s2 -= nums[i - k * 2 + 1];
                s3 -= nums[i - k + 1];
            }
        }

        ans
    }
}
