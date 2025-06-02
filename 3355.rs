//3355. Zero Array Transformation I

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let n = nums.len();
        let mut diff = vec![0; n];

        for query in &queries {
            let l = query[0] as usize;
            let r = query[1] as usize;

            diff[l] += 1;
            if r + 1 < n {
                diff[r + 1] -= 1;
            }
        }

        let mut cumulative_sum = 0;
        let mut res = vec![0; n];

        for i in 0..n {
            cumulative_sum += diff[i];
            res[i] = cumulative_sum;

            if res[i] < nums[i] {
                return false;
            }
        }

        true
    }
}
