//2269. Find the K-Beauty of a Number

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let numstr = num.to_string();
        let mut res = 0;

        for i in 0..=(numstr.len() as i32 - k) {
            if let Ok(cur) = numstr[i as usize..(i + k) as usize].parse::<i32>() {
                if cur != 0 && num % cur == 0 {
                    res += 1;
                }
            }
        }

        res
    }
}
