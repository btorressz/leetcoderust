//566. Reshape the Matrix
impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let r = r as usize;
        let c = c as usize;

        if m * n != r * c {
            return mat;
        }

        let mut flat = Vec::new();
        for row in mat.iter() {
            for &val in row.iter() {
                flat.push(val);
            }
        }

        let mut reshaped = Vec::new();
        for i in 0..r {
            let start = i * c;
            let end = start + c;
            reshaped.push(flat[start..end].to_vec());
        }

        reshaped
    }
}
