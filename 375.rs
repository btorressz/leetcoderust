//375. Guess Number Higher or Lower II

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let mut f = vec![vec![0; (n + 1) as usize]; (n + 1) as usize];
        
        for i in (1..n).rev() {
            for j in i + 1..=n {
                f[i as usize][j as usize] = j + f[i as usize][(j - 1) as usize];
                for k in i..j {
                    f[i as usize][j as usize] = f[i as usize][j as usize]
                        .min(k + std::cmp::max(f[i as usize][(k - 1) as usize], f[(k + 1) as usize][j as usize]));
                }
            }
        }

        f[1 as usize][n as usize]
    }
}
