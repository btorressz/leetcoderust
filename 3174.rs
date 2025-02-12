//3174. Clear Digits

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut result = Vec::new();

        for c in s.chars() {
            if c.is_alphabetic() {
                result.push(c);
            } else if !result.is_empty() {
                result.pop();
            }
        }

        result.into_iter().collect()
    }
}
