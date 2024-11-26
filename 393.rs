//393. UTF-8 Validation
impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut remaining_bytes = 0;
        data.into_iter().all(|byte| {
            let byte = byte as u8;
            if remaining_bytes == 0 {
                if byte >> 7 == 0b0 {
                    true 
                } else if byte >> 5 == 0b110 {
                    remaining_bytes = 1;
                    true 
                } else if byte >> 4 == 0b1110 {
                    remaining_bytes = 2;
                    true 
                } else if byte >> 3 == 0b11110 {
                    remaining_bytes = 3;
                    true 
                } else {
                    false 
                }
            } else {
                if byte >> 6 == 0b10 {
                    remaining_bytes -= 1;
                    true
                } else {
                    false
                }
            }
        }) && remaining_bytes == 0
    }
}
