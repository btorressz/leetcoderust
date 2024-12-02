//1455. Check If a Word Occurs As a Prefix of Any Word in a Sentence

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        // Split the sentence into words
        for (i, word) in sentence.split_whitespace().enumerate() {
            // Check if the word starts with the search_word
            if word.starts_with(&search_word) {
                // Return the 1-based index (enumerate starts at 0, so add 1)
                return (i + 1) as i32;
            }
        }
        // If no word starts with search_word, return -1
        -1
    }
}
