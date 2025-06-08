//2434. Using a Robot to Print the Lexicographically Smallest String

use std::collections::VecDeque;

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        let mut min_char_to_right = vec!['z'; n];
        min_char_to_right[n - 1] = chars[n - 1];

        for i in (0..n - 1).rev() {
            if chars[i] <= min_char_to_right[i + 1] {
                min_char_to_right[i] = chars[i];
            } else {
                min_char_to_right[i] = min_char_to_right[i + 1];
            }
        }

        let mut stack: VecDeque<char> = VecDeque::new();
        let mut res = String::new();

        for i in 0..n {
            stack.push_back(chars[i]);

            let min_char = if i + 1 < n {
                min_char_to_right[i + 1]
            } else {
                chars[i]
            };

            while let Some(&top) = stack.back() {
                if top <= min_char {
                    res.push(stack.pop_back().unwrap());
                } else {
                    break;
                }
            }
        }

        while let Some(c) = stack.pop_back() {
            res.push(c);
        }

        res
    }
}
