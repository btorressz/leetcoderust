impl Solution {
    pub fn to_hex(num: i32) -> String {
        // If the number is 0, return "0"
        if num == 0 {
            return "0".to_string();
        }

        // Convert the number to a 64-bit integer
        let mut num = num as i64; 
        let mut result = String::new();  // String to store the hex digits

        // If the number is negative, adjust it for 32-bit two's complement
        if num < 0 {
            num += 1 << 32; 
        }

        // Hex characters for 0-15
        let hex_chars = "0123456789abcdef";  

        // Convert the number to hex
        while num > 0 {
            // Get the last hex digit (lowest 4 bits)
            let digit = (num & 0xf) as usize;
            result.push(hex_chars.chars().nth(digit).unwrap());  // Add the hex digit to result
            
            // Shift right by 4 bits
            num >>= 4;
        }

        // Reverse the string and return
        result.chars().rev().collect()
    }
}
