//2911. Minimum Changes to Make K Semi-palindromes

use std::cmp::min;

impl Solution {
    pub fn minimum_changes(s: String, k: i32) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let k = k as usize;
        let inf = i32::MAX / 2;

        let mut g = vec![vec![inf; n + 1]; n + 1];

        for i in 1..=n {
            for j in i..=n {
                let m = j - i + 1;
                for d in 1..m {
                    if m % d == 0 {
                        let mut cnt = 0;
                        for l in 0..m {
                            let r = (m / d - 1 - l / d) * d + l % d;
                            if l >= r {
                                break;
                            }
                            if s[i - 1 + l] != s[i - 1 + r] {
                                cnt += 1;
                            }
                        }
                        g[i][j] = min(g[i][j], cnt);
                    }
                }
            }
        }

        let mut f = vec![vec![inf; k + 1]; n + 1];
        f[0][0] = 0;

        for i in 1..=n {
            for j in 1..=k {
                for h in 0..i {
                    f[i][j] = min(f[i][j], f[h][j - 1] + g[h + 1][i]);
                }
            }
        }

        f[n][k]
    }
}
