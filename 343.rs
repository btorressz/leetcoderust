//343. Integer Break
impl Solution {
    // Function to break `n` into parts to maximize the product
    pub fn integer_break(n: i32) -> i32 {
        
        // If n is 2, return 1 because 1 + 1 is the only split
        if n == 2 {
            return 1; 
        }
        
        // If n is 3, return 2 because 1 + 2 is the only split
        if n == 3 {
            return 2; 
        }
        
        // Start with product 1 and remaining value as n
        let mut product = 1;
        let mut remaining = n;
        
        // While remaining is greater than 4, multiply product by 3
        while remaining > 4 {
            product *= 3;
            remaining -= 3;
        }
        
        // Multiply product by the leftover remaining value
        product *= remaining;
        
        // Return the final product
        product
    }
}
