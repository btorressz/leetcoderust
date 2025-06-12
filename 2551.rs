//2551. Put Marbles in Bags

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        if k == 1 {
            return 0;
        }

        let mut pair_sums: Vec<i32> = Vec::with_capacity(weights.len() - 1);
        for i in 0..weights.len() - 1 {
            pair_sums.push(weights[i] + weights[i + 1]);
        }

        pair_sums.sort_unstable();

        let mut min_score: i64 = 0;
        let mut max_score: i64 = 0;

        for i in 0..(k - 1) as usize {
            min_score += pair_sums[i] as i64;
            max_score += pair_sums[pair_sums.len() - 1 - i] as i64;
        }

        max_score - min_score
    }
}
