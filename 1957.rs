impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut result = Vec::new();
        let chars: Vec<char> = s.chars().collect();

        for &ch in &chars {
            let n = result.len();
            if n >= 2 && result[n - 1] == ch && result[n - 2] == ch {
                continue; 
            }
            result.push(ch);
        }

        result.iter().collect()
    }
}
