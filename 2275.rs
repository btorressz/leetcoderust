impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        const MAX_BITS: usize = 24; 
        let mut bit_count = vec![0; MAX_BITS];

        for &num in &candidates {
            for bit in 0..MAX_BITS {
                if (num & (1 << bit)) != 0 {
                    bit_count[bit] += 1;
                }
            }
        }

        *bit_count.iter().max().unwrap()
    }
}
