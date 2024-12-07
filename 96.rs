//96. Unique Binary Search Trees

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![1; (n + 1) as usize];  
        
        for i in 2..=n {
            let mut count = 0;
            for left in 0..i {
                let left_trees = dp[left as usize];  
                let right_trees = dp[(i - left - 1) as usize];  
                count += left_trees * right_trees;  
            }
            dp[i as usize] = count;  
        }

        dp[n as usize]
    }
}
