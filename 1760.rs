//1760. Minimum Limit of Balls in a Bag

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        fn check(nums: &Vec<i32>, mx: i32, max_operations: i32) -> bool {
            nums.iter()
                .map(|&x| (x - 1) / mx)
                .sum::<i32>() <= max_operations
        }

        let max_num = *nums.iter().max().unwrap();
        let mut left = 1;
        let mut right = max_num;
        let mut result = right;  // Make `result` mutable

        while left <= right {
            let mid = left + (right - left) / 2;
            if check(&nums, mid, max_operations) {
                result = mid;  
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        result
    }
}
