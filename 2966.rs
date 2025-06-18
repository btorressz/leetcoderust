//2966. Divide Array Into Arrays With Max Difference

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut res = Vec::new();

        nums.sort();

        for i in (2..=n).step_by(3) {
            let group = vec![nums[i - 2], nums[i - 1], nums[i]];
            if group[2] - group[0] <= k {
                res.push(group);
            } else {
                return Vec::new(); 
            }
        }

        res
    }
}
