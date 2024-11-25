impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<(String, i32)> = Vec::new();
        let mut current_string = String::new();
        let mut current_num = 0;

        for ch in s.chars() {
            if ch.is_digit(10) {
                // Build the number for k
                current_num = current_num * 10 + ch.to_digit(10).unwrap() as i32;
            } else if ch == '[' {
                // Save current state and reset for new scope
                stack.push((current_string, current_num));
                current_string = String::new();
                current_num = 0;
            } else if ch == ']' {
                // Decode substring and append to previous state
                if let Some((prev_string, num)) = stack.pop() {
                    current_string = prev_string + &current_string.repeat(num as usize);
                }
            } else {
                // Append regular characters
                current_string.push(ch);
            }
        }

        current_string
    }
}
