//36. Valid Sudoku

use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let mut rows = HashSet::new();
            let mut columns = HashSet::new();
            let mut cube = HashSet::new();
            for j in 0..9 {
                // Check row
                let row_val = board[i][j];
                if row_val != '.' && !rows.insert(row_val) {
                    return false;
                }

                // check column
                let col_val = board[j][i];
                if col_val != '.' && !columns.insert(col_val) {
                    return false;
                }

                // Check cube (3x3 subgrid)
                let row_index = 3 * (i / 3);
                let col_index = 3 * (i % 3);
                let cube_val = board[row_index + j / 3][col_index + j % 3];
                if cube_val != '.' && !cube.insert(cube_val) {
                    return false;
                }
            }
        }
        true
    }
}
