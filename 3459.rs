//3459. Length of Longest V-Shaped Diagonal Segment
//Attempt two Successful 

/*
In my first attempt at solving the len_of_v_diagonal problem, I used a HashMap for memoization, where the key was a 5-tuple: (i, j, di, dj, turned). While functionally correct, this approach introduced significant 
performance issues due to excessive memory allocations and hashing overhead. Given the problem’s constraints (up to a 500x500 grid), this led to a Time Limit Exceeded (TLE) error. Additionally, memoization 
was less efficient because raw (di, dj) tuples resulted in redundant direction representations and non-uniform hashing.
To resolve this, I replaced the HashMap with a fixed-size 4D Vec, which drastically reduced lookup latency and memory management overhead. I also mapped each diagonal direction to a numeric index, enabling simple 
and fast array indexing. Finally, by pruning invalid paths early and avoiding redundant recursive calls, I made the solution significantly faster and scalable to large input sizes.
*/

impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        // Direction indices:
        // 0: (1, 1), 1: (-1, 1), 2: (1, -1), 3: (-1, -1)
        let dirs = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

        // Map current digit to next expected digit
        let next_digit = |d: i32| match d {
            1 => 2,
            2 => 0,
            0 => 2,
            _ => -1,
        };

        // Memo table: [i][j][direction][turned_flag] = length
        let mut memo = vec![vec![vec![vec![-1; 2]; 4]; n]; m];

        fn dfs(
            i: usize,
            j: usize,
            dir: usize,
            turned: usize,
            grid: &Vec<Vec<i32>>,
            memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
            dirs: &[(i32, i32)],
        ) -> i32 {
            if memo[i][j][dir][turned] != -1 {
                return memo[i][j][dir][turned];
            }

            let (di, dj) = dirs[dir];
            let m = grid.len() as i32;
            let n = grid[0].len() as i32;
            let i0 = i as i32;
            let j0 = j as i32;

            let cur = grid[i][j];
            let next = match cur {
                1 => 2,
                2 => 0,
                0 => 2,
                _ => -1,
            };

            let mut res = 1;

            // Continue straight
            let ni = i0 + di;
            let nj = j0 + dj;
            if ni >= 0 && ni < m && nj >= 0 && nj < n {
                let niu = ni as usize;
                let nju = nj as usize;
                if grid[niu][nju] == next {
                    res = res.max(1 + dfs(niu, nju, dir, turned, grid, memo, dirs));
                }
            }

            // Try to turn once (clockwise)
            if turned == 0 {
                let new_di = dirs[dir].1;
                let new_dj = -dirs[dir].0;

                let ni = i0 + new_di;
                let nj = j0 + new_dj;
                if ni >= 0 && ni < m && nj >= 0 && nj < n {
                    let niu = ni as usize;
                    let nju = nj as usize;
                    if grid[niu][nju] == next {
                        for (k, &(dx, dy)) in dirs.iter().enumerate() {
                            if dx == new_di && dy == new_dj {
                                res = res.max(1 + dfs(niu, nju, k, 1, grid, memo, dirs));
                                break;
                            }
                        }
                    }
                }
            }

            memo[i][j][dir][turned] = res;
            res
        }

        let mut max_len = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != 1 {
                    continue;
                }
                for d in 0..4 {
                    max_len = max_len.max(dfs(i, j, d, 0, &grid, &mut memo, &dirs));
                }
            }
        }

        max_len
    }
}






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
