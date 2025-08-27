//Attempt one wrong answer 

/*
use std::collections::HashMap;


impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        if m == 0 { return 0; }
        let n = grid[0].len();

        let directions = vec![(1, 1), (-1, 1), (1, -1), (-1, -1)];

        let next_digit = |d: i32| -> i32 {
            match d {
                1 => 2,
                2 => 0,
                0 => 2,
                _ => -1
            }
        };

        let mut memo = HashMap::new();

        fn within_bounds(i: isize, j: isize, m: usize, n: usize) -> bool {
            i >= 0 && j >= 0 && (i as usize) < m && (j as usize) < n
        }

        fn dfs(
            i: isize,
            j: isize,
            di: isize,
            dj: isize,
            turned: bool,
            grid: &Vec<Vec<i32>>,
            m: usize,
            n: usize,
            memo: &mut HashMap<(isize, isize, isize, isize, bool), i32>,
        ) -> i32 {
            if let Some(&val) = memo.get(&(i, j, di, dj, turned)) {
                return val;
            }

            let cur = grid[i as usize][j as usize];
            let successor = match cur {
                1 => 2,
                2 => 0,
                0 => 2,
                _ => -1,
            };

            let mut res = 1;

            let ni = i + di;
            let nj = j + dj;

            if within_bounds(ni, nj, m, n) && grid[ni as usize][nj as usize] == successor {
                res = 1 + dfs(ni, nj, di, dj, turned, grid, m, n, memo);
            }

            if !turned {
                let new_di = dj;
                let new_dj = -di;
                let ni = i + new_di;
                let nj = j + new_dj;
                if within_bounds(ni, nj, m, n) && grid[ni as usize][nj as usize] == successor {
                    res = res.max(1 + dfs(ni, nj, new_di, new_dj, true, grid, m, n, memo));
                }
            }

            memo.insert((i, j, di, dj, turned), res);
            res
        }

        let mut max_len = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != 1 {
                    continue;
                }

                for &(di, dj) in &directions {
                    max_len = max_len.max(dfs(
                        i as isize,
                        j as isize,
                        di,
                        dj,
                        false,
                        &grid,
                        m,
                        n,
                        &mut memo,
                    ));
                }
            }
        }

        max_len
    }
}




*/
