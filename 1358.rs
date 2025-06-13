//1358. Number of Substrings Containing All Three Characters

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let n = s.len();
        let s_bytes = s.as_bytes();
        let mut left = 0;
        let mut right = 0;
        let mut count = 0;
        let mut freq = [0; 3]; 

        while right < n {
            freq[(s_bytes[right] - b'a') as usize] += 1;

            while freq[0] > 0 && freq[1] > 0 && freq[2] > 0 {
                count += (n - right) as i32;
                freq[(s_bytes[left] - b'a') as usize] -= 1;
                left += 1;
            }

            right += 1;
        }

        count
    }
}
