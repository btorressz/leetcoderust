//2094. Finding 3-Digit Even Numbers

use std::collections::HashSet;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut ret = HashSet::new();
        let n = digits.len();

        for i in 0..n {
            if digits[i] == 0 {
                continue;
            }
            for j in 0..n {
                for k in 0..n {
                    if i != j && j != k && i != k && digits[k] % 2 == 0 {
                        let num = digits[i] * 100 + digits[j] * 10 + digits[k];
                        ret.insert(num);
                    }
                }
            }
        }

        let mut res: Vec<i32> = ret.into_iter().collect();
        res.sort_unstable();
        res
    }
}
