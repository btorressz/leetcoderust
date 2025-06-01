
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
