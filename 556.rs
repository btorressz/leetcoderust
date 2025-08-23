//556. Next Greater Element III

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits: Vec<char> = n.to_string().chars().collect();
        let len = digits.len();
        let mut i = len as i32 - 2;

        //  find first decreasing digit from right
        while i >= 0 && digits[i as usize] >= digits[i as usize + 1] {
            i -= 1;
        }

        if i < 0 {
            return -1;
        }

        //  find rightmost digit greater than digits[i]
        let mut j = len - 1;
        while digits[i as usize] >= digits[j] {
            j -= 1;
        }

        //  swap
        digits.swap(i as usize, j);

        //  reverse the suffix
        digits[(i as usize + 1)..].reverse();

        //  parse and check overflow
        if let Ok(val) = digits.iter().collect::<String>().parse::<i64>() {
            if val > i32::MAX as i64 {
                return -1;
            } else {
                return val as i32;
            }
        }

        -1
    }
}
