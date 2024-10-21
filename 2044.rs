impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let max_or = nums.iter().fold(0, |acc, &num| acc | num);
        
        let mut count = 0;

        for i in 1..(1 << n) {
            let mut current_or = 0;

            for j in 0..n {
                if (i & (1 << j)) != 0 {
                    current_or |= nums[j];
                }
            }

            if current_or == max_or {
                count += 1;
            }
        }
        
        count
    }
}
