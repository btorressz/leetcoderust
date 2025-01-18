//2429. Minimize XOR

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut count = num2.count_ones(); // Number of 1 bits needed
        let mut result = 0;

        //  Align with the most significant bits of num1
        for i in (0..31).rev() {
            if count == 0 {
                break;
            }
            if (num1 & (1 << i)) != 0 {
                result |= 1 << i; // Set this bit in the result
                count -= 1;
            }
        }

        // Fill remaining bits in the least significant positions
        for i in 0..31 {
            if count == 0 {
                break;
            }
            if (result & (1 << i)) == 0 {
                result |= 1 << i; // Set this bit in the result
                count -= 1;
            }
        }

        result
    }
}

/*
ATTEMPT ONE:
  impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut count = num2.count_ones(); // Count the number of 1 bits in num2
        let mut result = 0;

        // Cancel set bits from highest to lowest weight
        for i in (0..31).rev() {
            if (num1 & (1 << i)) != 0 {
                count -= 1;
                result += 1 << i;
            }
            if count == 0 {
                break;
            }
        }

        // Set bits lowest to highest weight
        for i in 0..32 {
            if (num1 & (1 << i)) == 0 {
                count -= 1;
                result += 1 << i;
            }
            if count == 0 {
                break;
            }
        }

        result
    }
}*/
