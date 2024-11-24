impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut total_sum = 0i64;
        let mut negative_count = 0;
        let mut min_abs_value = i32::MAX;

        for row in &matrix {
            for &value in row {
                let abs_value = value.abs();
                total_sum += abs_value as i64;
                if value < 0 {
                    negative_count += 1;
                }
                min_abs_value = min_abs_value.min(abs_value);
            }
        }

        if negative_count % 2 == 0 {
            total_sum
        } else {
            total_sum - 2 * min_abs_value as i64
        }
    }
}
