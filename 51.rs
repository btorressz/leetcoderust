//51. N-Queens

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut res = Vec::new();
        let mut board = vec![vec!['.'; n]; n];
        let mut col = vec![false; n];
        let mut dg = vec![false; 2 * n];
        let mut udg = vec![false; 2 * n];

        fn dfs(
            i: usize,
            n: usize,
            board: &mut Vec<Vec<char>>,
            col: &mut Vec<bool>,
            dg: &mut Vec<bool>,
            udg: &mut Vec<bool>,
            res: &mut Vec<Vec<String>>,
        ) {
            if i == n {
                res.push(board.iter().map(|row| row.iter().collect()).collect());
                return;
            }

            for j in 0..n {
                if !col[j] && !dg[i + j] && !udg[n - i + j] {
                    board[i][j] = 'Q';
                    col[j] = true;
                    dg[i + j] = true;
                    udg[n - i + j] = true;

                    dfs(i + 1, n, board, col, dg, udg, res);

                    board[i][j] = '.';
                    col[j] = false;
                    dg[i + j] = false;
                    udg[n - i + j] = false;
                }
            }
        }

        dfs(0, n, &mut board, &mut col, &mut dg, &mut udg, &mut res);
        res
    }
}
