//304. Range Sum Query 2D - Immutable

struct NumMatrix {
row_prefix_sum: Vec<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut row_prefix_sum = vec![vec![0; cols + 1]; rows]; // Row-wise prefix sums

        // compute the row-wise prefix sums
        for i in 0..rows {
            for j in 0..cols {
                row_prefix_sum[i][j + 1] = row_prefix_sum[i][j] + matrix[i][j];
            }
        }

        Self { row_prefix_sum }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let col1 = col1 as usize;
        let row2 = row2 as usize;
        let col2 = col2 as usize;

        let mut sum = 0;

        // sum up the rows between row1 and row2
        for row in row1..=row2 {
            sum += self.row_prefix_sum[row][col2 + 1] - self.row_prefix_sum[row][col1];
        }

        sum
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
