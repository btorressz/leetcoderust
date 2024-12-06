//1253. Reconstruct a 2-Row Binary Matrix

impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut upper = upper;
        let mut lower = lower;
        let n = colsum.len();
        
        // Check if the sum of colsum matches upper + lower, if not return an empty matrix
        if upper + lower != colsum.iter().sum::<i32>() {
            return vec![];
        }

        let mut matrix = vec![vec![0; n]; 2];

        // First pass: assign 1s where colsum is 2
        for c in 0..n {
            if colsum[c] == 2 {
                matrix[0][c] = 1;
                matrix[1][c] = 1;
                upper -= 1;
                lower -= 1;
                if upper < 0 || lower < 0 {
                    return vec![];
                }
            }
        }

        // Second pass: assign remaining 1s where colsum is 1
        for c in 0..n {
            if colsum[c] == 1 {
                if upper > 0 {
                    matrix[0][c] = 1;
                    upper -= 1;
                } else {
                    matrix[1][c] = 1;
                    lower -= 1;
                }
            }
        }

        matrix
    }
}
