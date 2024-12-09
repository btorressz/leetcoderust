//2597. The Number of Beautiful Subsets
//I used a recursive depth-first search (DFS) with backtracking and a frequency array (cnt) to solve the problem.

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = -1;
        let n = nums.len();
        let mut cnt = vec![0; 1010]; // This is analogous to Go's [1010]int (I first solved this problem in Go on leetcode back in May and wanted to try it solve it in Rust as well) 

        // Define the recursive dfs function
        fn dfs(i: usize, n: usize, nums: &Vec<i32>, k: i32, cnt: &mut Vec<i32>, ans: &mut i32) {
            if i >= n {
                *ans += 1;
                return;
            }
            // Recursive call for not including the current number
            dfs(i + 1, n, nums, k, cnt, ans);
            
            // Check the conditions for including the current number
            let ok1 = nums[i] + k >= cnt.len() as i32 || cnt[(nums[i] + k) as usize] == 0;
            let ok2 = nums[i] - k < 0 || cnt[(nums[i] - k) as usize] == 0;
            
            if ok1 && ok2 {
                cnt[nums[i] as usize] += 1;
                dfs(i + 1, n, nums, k, cnt, ans);
                cnt[nums[i] as usize] -= 1;
            }
        }

        dfs(0, n, &nums, k, &mut cnt, &mut ans);
        ans
    }
}
