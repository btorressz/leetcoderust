//3442. Maximum Difference Between Even and Odd Frequency I

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut freq = vec![0; 26];

        for c in s.chars() {
            freq[(c as u8 - b'a') as usize] += 1;
        }

        let mut min_even_freq = i32::MAX;
        let mut max_odd_freq = i32::MIN;

        for &f in &freq {
            if f % 2 == 0 && f != 0 {
                min_even_freq = min_even_freq.min(f);
            }

            if f % 2 == 1 {
                max_odd_freq = max_odd_freq.max(f);
            }
        }

        max_odd_freq - min_even_freq
    }
}
