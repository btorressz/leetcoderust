//324. Wiggle Sort II

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        nums.sort(); // sort the array first
        
        let n = nums.len();
        let mut result = vec![0; n];

        let mut left = (n + 1) / 2 - 1; // smaller half
        let mut right = n - 1;         // larger half

        // Reorder the sorted array into wiggle order
        for i in 0..n {
            if i % 2 == 0 {
                result[i] = nums[left];
                left -= 1;
            } else {
                result[i] = nums[right];
                right -= 1;
            }
        }

        // Copy the result back into nums
        nums.copy_from_slice(&result);
    }
}
