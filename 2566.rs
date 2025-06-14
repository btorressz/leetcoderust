//2566. Maximum Difference by Remapping a Digit

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let number = num.to_string();
        let chars: Vec<char> = number.chars().collect();

        let mut left_most_digit = '0';
        for &c in &chars {
            if c != '9' {
                left_most_digit = c;
                break;
            }
        }

        let s1: String = chars
            .iter()
            .map(|&c| if c == left_most_digit { '9' } else { c })
            .collect();
        let max_val = s1.parse::<i32>().unwrap();

        let left_most_digit_min = chars[0];
        let s2: String = chars
            .iter()
            .map(|&c| if c == left_most_digit_min { '0' } else { c })
            .collect();
        let min_val = s2.parse::<i32>().unwrap();

        max_val - min_val
    }
}
