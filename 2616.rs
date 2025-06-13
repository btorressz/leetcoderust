//2616. Minimize the Maximum Difference of Pairs

impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        nums.sort_unstable();
        let mut left = 0;
        let mut right = nums[nums.len() - 1] - nums[0];

        while left <= right {
            let mid = (left + right) / 2;
            if Self::can_find_k_pairs(mid, &nums, p) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    fn can_find_k_pairs(diff: i32, nums: &[i32], k: i32) -> bool {
        let mut count = 0;
        let mut i = 1;

        while i < nums.len() {
            if nums[i] - nums[i - 1] <= diff {
                count += 1;
                i += 1; 
            }
            i += 1;
        }

        count >= k
    }
}
