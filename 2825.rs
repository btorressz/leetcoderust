//2825. Make String a Subsequence Using Cyclic Increments
impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let mut i = 0;
        let str2_chars: Vec<char> = str2.chars().collect();
        
        for c in str1.chars() {
            let d = if c == 'z' { 'a' } else { (c as u8 + 1) as char };
            if i < str2.len() && (str2_chars[i] == c || str2_chars[i] == d) {
                i += 1;
            }
        }
        
        i == str2.len()
    }
}
