/* 
//ATTEMPT ONE 
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let m = m as usize;
        let n = n as usize;

        let mut matrix = vec![vec![0; n]; m];

        for op in &ops {
            let a = op[0] as usize;
            let b = op[1] as usize;

            for i in 0..a {
                for j in 0..b {
                    matrix[i][j] += 1;
                }
            }
        }
        
        let mut max_value = 0;
        let mut max_count = 0;

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] > max_value {
                    max_value = matrix[i][j];
                    max_count = 1;
                } else if matrix[i][j] == max_value {
                    max_count += 1;
                }
            }
        }
        max_count
    }
}

*/
