//1769. Minimum Number of Operations to Move All Balls to Each Box
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let s = boxes.as_bytes();
        let n = s.len();
        let mut val = vec![0; n];
        let mut cnt = 0;
        for i in 1..n {
            if s[i - 1] == b'1' {
                cnt += 1;
            }
            val[i] = val[i - 1] + cnt;
        }
        let mut sum = 0;
        cnt = 0;
        for i in (0..n - 1).rev() {
            if s[i + 1] == b'1' {
                cnt += 1;
            }
            sum += cnt;
            val[i] += sum;
        }
        val
    }
}
