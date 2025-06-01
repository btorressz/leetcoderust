//1931. Painting a Grid With Three Different Colors

//Correct answer 
use std::collections::HashMap;

impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let m = m as usize;
        let n = n as usize;

        fn generate_states(m: usize) -> Vec<Vec<i32>> {
            fn dfs(m: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
                if path.len() == m {
                    res.push(path.clone());
                    return;
                }
                for color in 0..3 {
                    if path.is_empty() || path.last().unwrap() != &color {
                        path.push(color);
                        dfs(m, path, res);
                        path.pop();
                    }
                }
            }

            let mut res = vec![];
            dfs(m, &mut vec![], &mut res);
            res
        }

        let states = generate_states(m);
        let mut dp: HashMap<Vec<i32>, i64> = HashMap::new();

        for state in &states {
            dp.insert(state.clone(), 1);
        }

        for _ in 1..n {
            let mut new_dp = HashMap::new();
            for (prev_state, &count) in dp.iter() {
                for state in &states {
                    let mut valid = true;
                    for i in 0..m {
                        if state[i] == prev_state[i] {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        *new_dp.entry(state.clone()).or_insert(0) += count;
                        *new_dp.get_mut(state).unwrap() %= MOD;
                    }
                }
            }
            dp = new_dp;
        }

        (dp.values().sum::<i64>() % MOD) as i32
    }
}

/*
attempt one wrong answer
impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let m = m as usize;
        let n = n as usize;

        let mut acc: i64 = 3;
        for _ in 0..(m - 1) {
            acc *= 2;
        }

        for _ in 0..(n - 1) {
            let mut same = acc;
            let mut diff = acc;
            for _ in 0..(m - 1) {
                let new_same = same + diff;
                let new_diff = 2 * diff + same;
                same = new_same % MOD;
                diff = new_diff % MOD;
            }
            acc = (same + diff) % MOD;
        }

        acc as i32
    }
}*/
