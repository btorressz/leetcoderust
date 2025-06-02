/*attempt one 
impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut res = Vec::new();

        if words.is_empty() || groups.is_empty() || words.len() != groups.len() {
            return res;
        }

        res.push(words[0].clone());

        for i in 1..words.len() {
            if groups[i] != groups[i - 1] {
                res.push(words[i].clone());
            }
        }

        res
    }
}



*/
