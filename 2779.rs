impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        // Compute the maximum value in nums and calculate the range size
        let m = *nums.iter().max().unwrap() + k * 2 + 2;
        
        // Create a difference array to accumulate values
        let mut d = vec![0; m as usize];
        
        // Populate the difference array based on nums
        for &x in &nums {
            d[x as usize] += 1;
            d[(x + k * 2 + 1) as usize] -= 1;
        }
        
        // running sum to find the maximum value of the accumulated array
        let mut max_beauty = 0;
        let mut acc = 0;
        for &value in &d {
            acc += value;
            max_beauty = max_beauty.max(acc);
        }
        
        max_beauty
    }
}
