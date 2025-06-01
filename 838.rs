//838. Push Dominoes

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let chars: Vec<char> = dominoes.chars().collect();
        let n = chars.len();
        let mut res = vec!['.'; n];

        let mut index = 0;
        while index < n && chars[index] == '.' {
            index += 1;
        }

        if index == n {
            return dominoes;
        }

        res[index] = chars[index];
        if chars[index] == 'L' {
            for i in 0..index {
                res[i] = 'L';
            }
        }

        let mut next_index = index + 1;
        while next_index < n {
            let next_char = chars[next_index];
            if next_char != '.' {
                res[next_index] = next_char;
                match (chars[index], next_char) {
                    ('R', 'L') => {
                        let mut l = index + 1;
                        let mut r = next_index - 1;
                        while l < r {
                            res[l] = 'R';
                            res[r] = 'L';
                            l += 1;
                            r -= 1;
                        }
                    }
                    ('R', 'R') => {
                        for i in index + 1..next_index {
                            res[i] = 'R';
                        }
                    }
                    ('L', 'L') => {
                        for i in index + 1..next_index {
                            res[i] = 'L';
                        }
                    }
                    _ => {}
                }
                index = next_index;
            }
            next_index += 1;
        }

        if chars[index] == 'R' {
            for i in index + 1..n {
                res[i] = 'R';
            }
        }

        res.into_iter().collect()
    }
}
