impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let k = k as usize;
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        // Count the total occurrences of 'a', 'b', and 'c'
        let mut total = [0; 3]; // Index 0 = 'a', 1 = 'b', 2 = 'c'
        for &ch in &chars {
            total[(ch as u8 - b'a') as usize] += 1;
        }

        // If any character occurs less than k times, it's impossible to satisfy the condition
        if total.iter().any(|&count| count < k) {
            return -1;
        }

        // Sliding window to find the largest valid subarray
        let target = [
            total[0] - k,
            total[1] - k,
            total[2] - k,
        ];

        let mut current = [0; 3]; // Current count of 'a', 'b', 'c' in the window
        let mut max_window = 0;
        let mut left = 0;

        for right in 0..n {
            // Add the current character to the window
            current[(chars[right] as u8 - b'a') as usize] += 1;

            // Shrink the window if any count exceeds its target
            while current[0] > target[0] || current[1] > target[1] || current[2] > target[2] {
                current[(chars[left] as u8 - b'a') as usize] -= 1;
                left += 1;
            }

            // Update the maximum window size
            max_window = max_window.max(right - left + 1);
        }

        // The result is the total string length minus the largest valid subarray length
        (n - max_window) as i32
    }
}
