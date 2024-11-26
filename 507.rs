//507 Perfect number
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        // Perfect numbers must be greater than 1
        if num <= 1 {
            return false;  
        }

        let mut sum = 1;  // Start with 1 
        let limit = (num as f64).sqrt() as i32;  // Only check up to sqrt(num)

        for i in 2..=limit {
            if num % i == 0 {  // If `i` divides num
                sum += i;  // Add `i`
                if i != num / i {  // add the other divisor
                    sum += num / i;
                }
            }
        }
        
        sum == num  // return true if divisors add up to num
    }
}
