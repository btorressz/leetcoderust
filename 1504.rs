//1504. Count Submatrices With All Ones
impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let h = mat.len();
        let w = mat[0].len();
        let mut dp_acc_of_1 = vec![vec![0; w]; h];

       // Fill in how many 1s are in a row going left in each row
          for y in 0..h {
            for x in 0..w {
                if x == 0 {
                    dp_acc_of_1[y][x] = mat[y][x];
                } else if mat[y][x] == 1 {
                    dp_acc_of_1[y][x] = dp_acc_of_1[y][x - 1] + 1;
                }
            }
        }

        // Count all rectangles that end at position (y, x)
        let mut counter_of_rectangle = 0;
        for y in 0..h {
            for x in 0..w {
                let mut min_width = dp_acc_of_1[y][x];
                for h_idx in (0..=y).rev() {
                    min_width = min_width.min(dp_acc_of_1[h_idx][x]);
                    if min_width == 0 {
                        break; // can't make a rectangle here
                    }
                    counter_of_rectangle += min_width;
                }
            }
        }

        counter_of_rectangle
    }
}
