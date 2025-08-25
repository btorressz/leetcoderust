//498. Diagonal Traverse
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        if m == 0 {
            return vec![];
        }

        let n = mat[0].len();
        if n == 0 {
            return vec![];
        }

        let mut ans = Vec::with_capacity(m * n);
        let (mut i, mut j, mut direction) = (0usize, 0usize, 1i32);
        let mut count = m * n;

        while count > 0 {
            ans.push(mat[i][j]);
            count -= 1;

            if direction == 1 {
                if i == 0 && j != n - 1 {
                    j += 1;
                    direction = -1;
                } else if j == n - 1 {
                    i += 1;
                    direction = -1;
                } else {
                    i = i.saturating_sub(1);
                    j += 1;
                }
            } else {
                if j == 0 && i != m - 1 {
                    i += 1;
                    direction = 1;
                } else if i == m - 1 {
                    j += 1;
                    direction = 1;
                } else {
                    i += 1;
                    j = j.saturating_sub(1);
                }
            }
        }

        ans
    }
}
