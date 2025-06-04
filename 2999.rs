//2999. Count the Number of Powerful Integers

impl Solution {
    const MAX_DIGITS: usize = 17;

    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, suffix: String) -> i64 {
        let suffix_val = suffix.parse::<i64>().unwrap();
        let finish_str = finish.to_string();
        let start_str = (start - 1).to_string();

        let upto_finish = if finish >= suffix_val {
            Self::solve_up_to(&finish_str, limit, &suffix)
        } else {
            0
        };

        let upto_start = if suffix_val < start {
            Self::solve_up_to(&start_str, limit, &suffix)
        } else {
            0
        };

        upto_finish - upto_start
    }

    fn solve_up_to(num_str: &str, limit: i32, suffix: &str) -> i64 {
        let mut dp = vec![vec![-1; 2]; Self::MAX_DIGITS];
        let digits = num_str.len();
        let res = Self::count_valid_numbers(num_str, 0, digits, true, limit, suffix, &mut dp);

        if Self::check_subtract(num_str, suffix, limit) {
            res - 1
        } else {
            res
        }
    }

    fn count_valid_numbers(
        num_str: &str,
        idx: usize,
        max_digits: usize,
        is_tight: bool,
        limit: i32,
        suffix: &str,
        dp: &mut Vec<Vec<i64>>,
    ) -> i64 {
        if idx == max_digits {
            return 1;
        }
        let tight_idx = if is_tight { 1 } else { 0 };
        if dp[idx][tight_idx] != -1 {
            return dp[idx][tight_idx];
        }

        let suffix_len = suffix.len();
        let (low, high) = if idx >= max_digits - suffix_len {
            let suffix_idx = idx - (max_digits - suffix_len);
            let fixed_digit = suffix.chars().nth(suffix_idx).unwrap().to_digit(10).unwrap() as i32;
            (fixed_digit, fixed_digit)
        } else {
            let high = if is_tight {
                (num_str.chars().nth(idx).unwrap() as u8 - b'0') as i32
            } else {
                limit
            };
            (0, high.min(limit))
        };

        let mut tot = 0;
        for digit in low..=high {
            let next_tight = is_tight && (digit == (num_str.chars().nth(idx).unwrap() as u8 - b'0') as i32);
            tot += Self::count_valid_numbers(
                num_str,
                idx + 1,
                max_digits,
                next_tight,
                limit,
                suffix,
                dp,
            );
        }

        dp[idx][tight_idx] = tot;
        tot
    }

    fn check_subtract(num_str: &str, suffix: &str, limit: i32) -> bool {
        let num_digits = num_str.len();
        if num_digits < suffix.len() {
            return false;
        }

        let suffix_of_num = &num_str[num_digits - suffix.len()..];
        let subtract = suffix_of_num.parse::<i64>().unwrap() < suffix.parse::<i64>().unwrap();

        if subtract {
            for ch in num_str.chars().take(num_digits - suffix.len()) {
                if (ch as u8 - b'0') as i32 > limit {
                    return false;
                }
            }
        }

        subtract
    }
}
