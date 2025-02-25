//52. N-Queens II

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut cols = vec![false; n];
        let mut dg = vec![false; 2 * n]; 
        let mut udg = vec![false; 2 * n];
        let mut res = 0; 

        fn backtrack(
            i: usize,
            n: usize,
            cols: &mut Vec<bool>,
            dg: &mut Vec<bool>,
            udg: &mut Vec<bool>,
            res: &mut i32,
        ) {
            if i == n {
                *res += 1;
                return;
            }

            for j in 0..n {
                let a = i + j; 
                let b = i + n - j; 
                if cols[j] || dg[a] || udg[b] {
                    continue;
                }

                cols[j] = true;
                dg[a] = true;
                udg[b] = true;

                backtrack(i + 1, n, cols, dg, udg, res);

                cols[j] = false;
                dg[a] = false;
                udg[b] = false;
            }
        }

        backtrack(0, n, &mut cols, &mut dg, &mut udg, &mut res);

        res
    }
}
