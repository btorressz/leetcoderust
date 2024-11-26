impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut result = vec![];
        let mut last_group = -1; 

        for (i, &group) in groups.iter().enumerate() {
            if group != last_group {
                result.push(words[i].clone());
                last_group = group;
            }
        }

        result
    }
}
