//1573. Number of Ways to Split a String

impl Solution {
    pub fn num_ways(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let ones_total = s.chars().filter(|&c| c == '1').count() as i64;
        let n = s.len() as i64;

        // If the number of '1's is not divisible by 3, return 0
        let (ones_per_part, remainder) = (ones_total / 3, ones_total % 3);
        if remainder != 0 {
            return 0;
        }

        // If there are no '1's, compute the number of ways to split into 3 parts
        if ones_per_part == 0 {
            return (((n - 1) * (n - 2) / 2) % MOD) as i32;
        }

        // Function to find the first index where sum of '1's reaches x
        let find = |target: i64| -> usize {
            let mut count = 0;
            for (i, c) in s.chars().enumerate() {
                if c == '1' {
                    count += 1;
                }
                if count == target {
                    return i;
                }
            }
            0 // should never be reaches b/c assumed valid point
        };

        let i1 = find(ones_per_part) as i64;
        let i2 = find(ones_per_part + 1) as i64;
        let j1 = find(ones_per_part * 2) as i64;
        let j2 = find(ones_per_part * 2 + 1) as i64;

        ((i2 - i1) * (j2 - j1) % MOD) as i32
    }
}
