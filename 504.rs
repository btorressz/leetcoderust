//504 Base 7
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        // return "0" if num is 0
        if num == 0 {
            return "0".to_string();
        }

        // check if num is negative
        let is_negative = num < 0;
        // use absolute value of num
        let mut num = num.abs(); 

        // store digits
        let mut result = Vec::new();

        // convert to base 7
        while num > 0 {
            result.push((num % 7).to_string());  // remainder
            num /= 7;  // divide by 7
        }

        // reverse digits
        let base7 = result.into_iter().rev().collect::<String>();

        // add "-" if negative
        if is_negative {
            format!("-{}", base7)
        } else {
            base7  // return result
        }
    }
}
