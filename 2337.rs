//2337. Move Pieces to Obtain a String

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let n = start.len();
        let start_bytes = start.as_bytes();  // Convert start to a byte slice for O(1) access
        let target_bytes = target.as_bytes();  // Convert target to a byte slice for O(1) access
        
        let mut i = 0;
        let mut j = 0;
        
        while i < n || j < n {
            // Skip the underscores in the start string
            while i < n && start_bytes[i] == b'_' {
                i += 1;
            }
            // Skip the underscores in the target string
            while j < n && target_bytes[j] == b'_' {
                j += 1;
            }

            if i >= n && j >= n {
                return true;  // Both strings have been fully processed
            }
            if i >= n || j >= n || start_bytes[i] != target_bytes[j] {
                return false;  // Mismatch in characters or one string is exhausted
            }
            if start_bytes[i] == b'L' && i < j {
                return false;  // 'L' cannot move to the right
            }
            if start_bytes[i] == b'R' && i > j {
                return false;  // 'R' cannot move to the left
            }

            // Move to the next character in both strings
            i += 1;
            j += 1;
        }

        true
    }
}
