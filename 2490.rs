impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words: Vec<&str> = sentence.split_whitespace().collect();
        let n = words.len();

        for i in 0..n {
            let current_word = words[i];
            let next_word = words[(i + 1) % n]; 
            if current_word.chars().last().unwrap() != next_word.chars().next().unwrap() {
                return false;
            }
        }

        true
    }
}
