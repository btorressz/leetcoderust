//1408. String Matching in an Array

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut val = Vec::new();
        let w = words.len();
    for i in 0..w {
            for j in 0..w {
                if i != j && words[j].contains(&words[i]) {
                    val.push(words[i].clone());
                    break;
                }
            }
        }
        val
    }
}
