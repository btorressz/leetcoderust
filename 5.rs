//5. Longest Palindromic Substring

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let mut f = vec![vec![true; n]; n];
        let mut k = 0;
        let mut mx = 1;
        let chars: Vec<char> = s.chars().collect();

        for i in (0..n - 1).rev() {
            for j in i + 1..n {
                f[i][j] = false;
                if chars[i] == chars[j] {
                    f[i][j] = f[i + 1][j - 1] || j - i == 1;
                    if f[i][j] && mx < j - i + 1 {
                        k = i;
                        mx = j - i + 1;
                    }
                }
            }
        }

        s[k..k + mx].to_string()
    }
}
