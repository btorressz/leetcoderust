//1752. Check if Array Is Sorted and Rotated

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let sorted_nums: Vec<i32> = {
            let mut temp = nums.clone();
            temp.sort();
            temp
        };

        let doubled_nums: Vec<i32> = nums.iter().chain(nums.iter()).cloned().collect();

        doubled_nums.windows(sorted_nums.len()).any(|window| window == sorted_nums)
    }
}
