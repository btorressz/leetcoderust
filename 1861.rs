impl Solution {
    pub fn rotate_the_box(input_box: Vec<Vec<char>>) -> Vec<Vec<char>> {
        if input_box.is_empty() || input_box[0].is_empty() {
            return vec![]; // Handle empty input
        }
        
        let m = input_box.len();
        let n = input_box[0].len();
        
        // Create a result array for "gravity effect" within rows
        let mut result = vec![vec!['.'; n]; m];
        
        for i in 0..m {
            let mut empty_pos = n - 1;
            for j in (0..n).rev() {
                match input_box[i][j] {
                    '#' => {
                        result[i][empty_pos] = '#';
                        empty_pos -= 1;
                    }
                    '*' => {
                        result[i][j] = '*';
                        empty_pos = j - 1;
                    }
                    _ => {} // No action needed for '.'
                }
            }
        }
        
        // Rotate the "gravity-affected" result array
        let mut rotated_result = vec![vec!['.'; m]; n];
        for i in 0..m {
            for j in 0..n {
                rotated_result[j][m - 1 - i] = result[i][j];
            }
        }
        
        rotated_result
    }
}
