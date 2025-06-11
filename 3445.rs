//3445. Maximum Difference Between Even and Odd Frequency II

impl Solution {
    fn get_state(a_count: i32, b_count: i32) -> usize {
        let a_bit = a_count & 1;
        let b_bit = b_count & 1;

        match (a_bit, b_bit) {
            (0, 0) => 0, // even even
            (0, 1) => 1, // even odd
            (1, 0) => 2, // odd even
            _ => 3,      // odd odd
        }
    }

    pub fn max_difference(s: String, k: i32) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let n = s_chars.len();
        let mut res = i32::MIN;

        for a in '0'..='4' {
            for b in '0'..='4' {
                if a == b {
                    continue;
                }

                let mut min_prev_diff = vec![i32::MAX; 4];
                let (mut count_a, mut count_b) = (0, 0);
                let (mut prev_a, mut prev_b) = (0, 0);
                let (mut l, mut r) = (-1, 0);

                while r < n {
                    if s_chars[r] == a {
                        count_a += 1;
                    }
                    if s_chars[r] == b {
                        count_b += 1;
                    }

                    while r as i32 - l >= k && (count_b - prev_b) >= 2 && (count_a - prev_a) >= 1 {
                        let left_state = Self::get_state(prev_a, prev_b);
                        min_prev_diff[left_state] = min_prev_diff[left_state].min(prev_a - prev_b);

                        l += 1;

                        if s_chars[l as usize] == a {
                            prev_a += 1;
                        }
                        if s_chars[l as usize] == b {
                            prev_b += 1;
                        }
                    }

                    let right_state = Self::get_state(count_a, count_b);
                    let target_state = right_state ^ 2;
                    let min_diff_val = min_prev_diff[target_state];

                    if min_diff_val != i32::MAX {
                        res = res.max((count_a - count_b) - min_diff_val);
                    }

                    r += 1;
                }
            }
        }

        res
    }
}
