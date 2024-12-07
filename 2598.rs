//2598. Smallest Missing Non-negative Integer After Operations

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        // Initialize a vector to count occurrences of each possible remainder (mod value)
        let mut counter = vec![0; value as usize];

        // Count the occurrences of each mod value
        for &num in &nums {
            // Ensure positive remainders and cast to usize for indexing
            counter[((num % value + value) % value) as usize] += 1;
        }

        // Find the minimum occurrence count across all remainders
        let min_occurance = *counter.iter().min().unwrap();

        // Find the first remainder (mod value) with the minimum occurrence
        let mut selected = 0;
        for i in 0..value {
            if counter[i as usize] == min_occurance {
                selected = i;
                break;
            }
        }

        // Calculate the result as the number of times the remainder can appear
        // (min_occurance) multiplied by the value, plus the selected remainder
        value * min_occurance + selected
    }
}
