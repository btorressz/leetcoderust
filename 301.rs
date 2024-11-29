//301. Remove Invalid Parentheses

use std::collections::HashSet;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        fn dfs(
            i: usize,
            l: i32,
            r: i32,
            lcnt: i32,
            rcnt: i32,
            t: String,
            s: &Vec<char>,
            n: usize,
            ans: &mut HashSet<String>,
        ) {
            if i == n {
                if l == 0 && r == 0 {
                    ans.insert(t);
                }
                return;
            }
            if (n - i) < (l + r) as usize || lcnt < rcnt {
                return;
            }
            if s[i] == '(' && l > 0 {
                dfs(i + 1, l - 1, r, lcnt, rcnt, t.clone(), s, n, ans);
            } else if s[i] == ')' && r > 0 {
                dfs(i + 1, l, r - 1, lcnt, rcnt, t.clone(), s, n, ans);
            }
            let mut next_t = t.clone();
            next_t.push(s[i]);
            dfs(
                i + 1,
                l,
                r,
                lcnt + if s[i] == '(' { 1 } else { 0 },
                rcnt + if s[i] == ')' { 1 } else { 0 },
                next_t,
                s,
                n,
                ans,
            );
        }
        // Count the number of invalid '(' and ')' to be removed
        let mut l = 0;
        let mut r = 0;
        for c in s.chars() {
            if c == '(' {
                l += 1;
            } else if c == ')' {
                if l > 0 {
                    l -= 1;
                } else {
                    r += 1;
                }
            }
        }

        let mut ans = HashSet::new();
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        dfs(0, l, r, 0, 0, String::new(), &chars, n, &mut ans);

        // Convert the HashSet to a Vec and return
        ans.into_iter().collect()
    }
}
