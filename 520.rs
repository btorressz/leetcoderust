//520. Detect Capital

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let cnt = word.chars().filter(|c| c.is_uppercase()).count();
        cnt == 0 || cnt == word.len() || (cnt == 1 && word.chars().next().unwrap().is_uppercase())
    }
}
