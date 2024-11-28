//421. Maximum XOR of Two Numbers in an Array
use std::collections::HashSet;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut max_xor = 0; // To store the maximum XOR result
        let mut mask = 0; // Mask to extract prefixes

        // Iterate from the most significant bit to the least significant bit
        for i in (0..31).rev() {
            mask |= 1 << i; // Update the mask to include the current bit
            let mut prefixes = HashSet::new();

            // Collect all prefixes with the current mask
            for &num in &nums {
                prefixes.insert(num & mask);
            }

            // Tentative maximum XOR with the current bit set to 1
            let expected_xor = max_xor | (1 << i);

            // Check if any two prefixes can produce the expected_xor
            let mut found = false;
            for &prefix in &prefixes {
                if prefixes.contains(&(prefix ^ expected_xor)) {
                    found = true;
                    break;
                }
            }

            // If found, set the current bit in max_xor
            if found {
                max_xor = expected_xor;
            }
        }

        max_xor
    }
}
