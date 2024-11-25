impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
          let mut k = 0;
        let mut repeated_word = String::new();

        while sequence.contains(&(repeated_word.clone() + &word)) {
            repeated_word += &word;
            k += 1;
        }

        k
    }
}
