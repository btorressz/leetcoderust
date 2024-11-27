//575. Distribute Candies
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        const OFFSET: usize = 100_000; 
        let mut seen = vec![false; 200_001]; 
        let mut unique_count = 0;

        for &candy in &candy_type {
            let index = (candy + OFFSET as i32) as usize;
            if !seen[index] {
                seen[index] = true;
                unique_count += 1;
            }
        }

        let max_candies = candy_type.len() / 2;
        unique_count.min(max_candies) as i32
    }
}
