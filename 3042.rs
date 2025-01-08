//3042. Count Prefix and Suffix Pairs I

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut val = 0;
        let n = words.len();
        
        for i in 0..n {
            for j in (i + 1)..n {
                if words[j].starts_with(&words[i]) && words[j].ends_with(&words[i]) {
                    val += 1;
                }
            }
        }
        
        val
    }
}
