//420. Strong Password Checker

/*
impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let n = password.len();
        let mut missing_types = 3;
        let mut lower = false;
        let mut upper = false;
        let mut digit = false;

        // Count missing character types
        for ch in password.chars() {
            if ch.is_lowercase() { lower = true; }
            if ch.is_uppercase() { upper = true; }
            if ch.is_digit(10) { digit = true; }
        }
        missing_types -= (lower as i32 + upper as i32 + digit as i32);

        // Count repetitive sequences
        let mut repeat_lens = Vec::new();
        let chars: Vec<char> = password.chars().collect();
        let mut i = 0;

        while i < n {
            let mut j = i;
            while j < n && chars[j] == chars[i] {
                j += 1;
            }
            if j - i >= 3 {
                repeat_lens.push(j - i);
            }
            i = j;
        }

        // Handle different length scenarios
        if n < 6 {
            return std::cmp::max(6 - n as i32, missing_types);
        }

        if n <= 20 {
            let mut replace = 0;
            for &len in &repeat_lens {
                replace += len / 3;
            }
            return std::cmp::max(replace as i32, missing_types);
        }

        // Complex case: > 20 characters
        let delete = n - 20;
        let mut replace = 0;

        // Try to use deletions to break repetitive sequences
        let mut temp_repeat_lens = repeat_lens.clone();
        let mut remaining_delete = delete;

        // Reduce sequences divisible by 3 first
        for i in 0..temp_repeat_lens.len() {
            if remaining_delete > 0 && temp_repeat_lens[i] % 3 == 0 {
                let remove = std::cmp::min(remaining_delete, temp_repeat_lens[i] / 3 * 3);
                temp_repeat_lens[i] -= remove;
                remaining_delete -= remove;
            }
        }

        // Reduce sequences where len % 3 == 1
        for i in 0..temp_repeat_lens.len() {
            if remaining_delete > 0 && temp_repeat_lens[i] % 3 == 1 {
                let remove = std::cmp::min(remaining_delete, temp_repeat_lens[i] - 2);
                temp_repeat_lens[i] -= remove;
                remaining_delete -= remove;
            }
        }

        // Reduce remaining sequences
        for &len in &temp_repeat_lens {
            replace += len / 3;
        }

        // Total changes = deletions + remaining replacements + missing types
        delete as i32 + std::cmp::max(replace as i32, missing_types)
    }
}


*/

/* impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        // Helper function to count the types of characters (lowercase, uppercase, digits)
        fn count_types(s: &str) -> i32 {
            let mut has_lower = false;
            let mut has_upper = false;
            let mut has_digit = false;
            
            for ch in s.chars() {
                if ch.is_lowercase() {
                    has_lower = true;
                } else if ch.is_uppercase() {
                    has_upper = true;
                } else if ch.is_digit(10) {
                    has_digit = true;
                }
            }

            // Return the count of missing character types (0, 1, 2, or 3)
            (has_lower as i32) + (has_upper as i32) + (has_digit as i32)
        }

        let n = password.len() as i32; // Cast `n` to `i32`
        let types = count_types(&password);
        
        // Case 1: Password is too short
        if n < 6 {
            return std::cmp::max(6 - n, 3 - types);  // Both arguments are now i32
        }
        
        // Case 2: Password length is between 6 and 20 characters
        if n <= 20 {
            let mut replace = 0;
            let mut consecutive_count = 2; // Count of current repeated characters
            let mut prev_char = '\0';
            
            for curr_char in password.chars() {
                if curr_char == prev_char {
                    consecutive_count += 1;
                } else {
                    consecutive_count = 1;
                }

                if consecutive_count >= 3 {
                    replace += 1;
                    consecutive_count = 1; // Reset count after replacement
                }

                prev_char = curr_char;
            }

            // Return the maximum between missing types (3 - types) and the number of replacements needed
            return std::cmp::max(replace, 3 - types);  // Both arguments are now i32
        }
        
        // Case 3: Password length is greater than 20
        let mut replace = 0;
        let mut consecutive_count = 2;
        let mut prev_char = '\0';
        let mut excess_length = n - 20;
        let mut remove2 = 0; // Tracks sequences for removal
        
        let mut password_chars = password.chars().collect::<Vec<_>>();
        
        for i in 0..password_chars.len() {
            if i > 0 && password_chars[i] == password_chars[i-1] {
                consecutive_count += 1;
            } else {
                consecutive_count = 1;
            }

            if consecutive_count >= 3 {
                replace += 1;
                consecutive_count = 1;
            }
        }
        
        // Remove characters to meet length requirement
        let use2 = std::cmp::min(std::cmp::min(replace, remove2), excess_length / 2);  // Corrected to two arguments
        replace -= use2;
        excess_length -= use2 * 2;

        let use3 = std::cmp::min(replace, excess_length / 3);
        replace -= use3;
        excess_length -= use3 * 3;

        // After adjusting excess characters, calculate the remaining number of changes
        (n - 20) + std::cmp::max(replace, 3 - types)  // Both arguments are now i32
    }
}
*/
