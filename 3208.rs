//3208. Alternating Groups II

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let n = colors.len();
        let mut count = 0;
        let mut left = 0;
        let limit = n + k as usize - 1;

        while left < n {
            let mut right = left + 1;

            while right < limit && colors[(right - 1) % n] != colors[right % n] {
                right += 1;
            }

            let length = right - left;
            if length >= k as usize {
                count += (length - k as usize + 1) as i32;
            }

            left = right;
        }

        count
    }
}
