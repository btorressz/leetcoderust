//2901. Longest Unequal Adjacent Groups Subsequence II

//Attempt two successful attempt

impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = words.len();
        let mut dp = vec![1; n];
        let mut prev = vec![-1; n];

        fn hamming_dist(a: &str, b: &str) -> i32 {
            if a.len() != b.len() {
                return -1;
            }
            a.chars()
                .zip(b.chars())
                .filter(|(x, y)| x != y)
                .count() as i32
        }

        for i in 0..n {
            for j in 0..i {
                if groups[i] != groups[j]
                    && words[i].len() == words[j].len()
                    && hamming_dist(&words[i], &words[j]) == 1
                    && dp[j] + 1 > dp[i]
                {
                    dp[i] = dp[j] + 1;
                    prev[i] = j as i32;
                }
            }
        }

        // get index of max dp
        let mut max_idx = 0;
        for i in 1..n {
            if dp[i] > dp[max_idx] {
                max_idx = i;
            }
        }

        // backtrack to build the subsequence
        let mut res = Vec::new();
        let mut idx = max_idx as i32;
        while idx != -1 {
            res.push(words[idx as usize].clone());
            idx = prev[idx as usize];
        }

        res.reverse();
        res
    }
}

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
