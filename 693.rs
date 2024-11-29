//693. Binary Number with Alternating Bits
impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        let mut prev = -1; // Initialize previous bit to an invalid value (-1)

        while n > 0 {
            let curr = n & 1; // Extract the least significant bit 
            if prev == curr {
                return false; // If the current bit is the same as the previous bit, return false
            }
            prev = curr; // Update the previous bit
            n >>= 1; // Shift n right by 1 to process the next bit
        }

        true // If  process all bits without conflict, return true
    }
}
