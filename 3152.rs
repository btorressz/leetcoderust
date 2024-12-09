//3152. Special Array II

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = nums.len();
        let mut d: Vec<usize> = (0..n).collect(); 

        for i in 1..n {
            if nums[i] % 2 != nums[i - 1] % 2 {
                d[i] = d[i - 1];
            }
        }

        queries.into_iter().map(|query| {
            let (f, t) = (query[0] as usize, query[1] as usize);
            d[t] <= f
        }).collect()
    }
}
