impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        // Map to store the last occurrence of each character
        let mut last_occurrence = [0; 26];
        for (i, c) in s.chars().enumerate() {
            last_occurrence[(c as u8 - b'a') as usize] = i;
        }

        // Result vector to store sizes of partitions
        let mut result = Vec::new();
        let mut start = 0; // Start of the current partition
        let mut end = 0;   // End of the current partition

        // Iterate through the string to form partitions
        for (i, c) in s.chars().enumerate() {
            end = end.max(last_occurrence[(c as u8 - b'a') as usize]); // Update partition end
            if i == end { // If end of partition is reached
                result.push((end - start + 1) as i32); // Add partition size
                start = i + 1; // Start new partition
            }
        }

        result
    }
}
