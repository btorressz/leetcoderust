//  2381. Shifting Letters II

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let mut d = vec![0; n + 1]; 

        for shift in shifts {
            let i = shift[0] as usize;
            let j = shift[1] as usize;
            let v = if shift[2] == 0 { -1 } else { 1 }; 
            d[i] += v;
            d[j + 1] -= v; 
        }

        let mut accumulated_shift = 0;
        let mut result = String::with_capacity(n);

        for (i, &ch) in s.as_bytes().iter().enumerate() {
            accumulated_shift += d[i];
            let new_char = ((ch - b'a') as i32 + accumulated_shift % 26 + 26) % 26 + b'a' as i32;
            result.push(new_char as u8 as char);
        }

        result
    }
}

