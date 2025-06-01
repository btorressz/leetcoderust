//1128. Number of Equivalent Domino Pairs

use std::collections::HashMap;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut count: HashMap<i32, i32> = HashMap::new();

        for domino in dominoes.iter() {
            let a = domino[0];
            let b = domino[1];
            let key = a.min(b) * 10 + a.max(b);
            if let Some(&val) = count.get(&key) {
                res += val;
            }
            *count.entry(key).or_insert(0) += 1;
        }

        res
    }
}
