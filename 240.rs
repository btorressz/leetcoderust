impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut row = 0;
        let mut col = matrix[0].len() as i32 - 1; 
        while row < matrix.len() && col >= 0 {
            let current = matrix[row][col as usize];
            if current == target {
                return true; 
            } else if current > target {
                col -= 1; 
            } else {
                row += 1; 
            }
        }

        false 
    }
}
