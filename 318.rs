//318. Maximum Product of Word Lengths
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let n = words.len();  
        let mut max_product = 0;  
        let mut bitmasks = Vec::with_capacity(n);
        for word in &words {
            let mut bitmask = 0;  
            for c in word.chars() {
                bitmask |= 1 << (c as u8 - b'a');
            }
            bitmasks.push((bitmask, word.len()));
        }
        for i in 0..n {
            for j in i + 1..n {
                if bitmasks[i].0 & bitmasks[j].0 == 0 {
                    max_product = max_product.max(bitmasks[i].1 * bitmasks[j].1);
                }
            }
        }
        max_product as i32  
    }
}
