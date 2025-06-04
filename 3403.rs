//3403. Find the Lexicographically Largest String From the Box I

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }

        let n = word.len();
        let chars: Vec<char> = word.chars().collect();
        let longest_pos_len = n - (num_friends - 1) as usize;

        let mut res = String::new();

        for i in 0..n {
            let pos_len = std::cmp::min(longest_pos_len, n - i);
            if i + pos_len > n {
                continue;
            }
            let sub: String = chars[i..i + pos_len].iter().collect();
            if sub > res {
                res = sub;
            }
        }

        res
    }
}
