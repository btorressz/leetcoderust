//482. License Key Formatting

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let length = s.len();
        let mut remaining_chars_in_segment = (length - s.matches('-').count()) % k as usize;
        if remaining_chars_in_segment == 0 {
            remaining_chars_in_segment = k as usize;
        }
        let mut formatted_chars = Vec::new();
        
        for (i, c) in s.chars().enumerate() {
            if c == '-' {
                continue;
            }
            formatted_chars.push(c.to_ascii_uppercase());
            if remaining_chars_in_segment == 1 && i != length - 1 {
                formatted_chars.push('-');
                remaining_chars_in_segment = k as usize;
            } else {
                remaining_chars_in_segment -= 1;
            }
        }
        
        if let Some(last) = formatted_chars.last() {
            if *last == '-' {
                formatted_chars.pop();
            }
        }
        
        formatted_chars.into_iter().collect()
    }
}
