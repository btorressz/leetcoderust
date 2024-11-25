impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // initialize slow and fast pointers
        let mut slow = nums[0];
        let mut fast = nums[0];

        // detect a cycle using the tortoise and hare method
        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
            if slow == fast {
                break; // cycle detected
            }
        }

        // find the entry point of the cycle
        let mut slow2 = nums[0];
        while slow != slow2 {
            slow = nums[slow as usize];
            slow2 = nums[slow2 as usize];
        }

        // return the duplicate number
        slow
    }
}
