//368. Largest Divisible Subset

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        // Sort the input numbers
        nums.sort();
        let n = nums.len();
        if n == 0 {
            return vec![];
        }

        // Create a DP array `f` where `f[i]` represents the size of the largest divisible subset ending at index `i`
        let mut f = vec![1; n];
        let mut prev = vec![-1; n]; // To keep track of the previous index for reconstructing the subset
        let mut max_size = 0;
        let mut max_index = 0;

        // Fill the DP table
        for i in 0..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && f[i] < f[j] + 1 {
                    f[i] = f[j] + 1;
                    prev[i] = j as i32; // Store the index of the previous element in the subset
                }
            }
            // Update the maximum size and the corresponding index
            if f[i] > max_size {
                max_size = f[i];
                max_index = i;
            }
        }

        // Reconstruct the subset from the `prev` array
        let mut ans = Vec::new();
        let mut current = max_index as i32;
        while current != -1 {
            ans.push(nums[current as usize]);
            current = prev[current as usize];
        }

        // Since the subset is constructed in reverse, reverse it before returning
        ans.reverse();
        ans
    }
}
