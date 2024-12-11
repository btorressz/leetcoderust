//506. Relative Ranks
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let n = score.len(); // Number of scores
        let mut idx: Vec<usize> = (0..n).collect(); // Indices

        idx.sort_by(|&a, &b| score[b].cmp(&score[a])); // Sort indices by scores (desc)

        let top3 = vec!["Gold Medal".to_string(), "Silver Medal".to_string(), "Bronze Medal".to_string()]; // Top 3 medals

        let mut ans = vec![String::new(); n]; // Result placeholder

        for i in 0..n {
            if i < 3 {
                ans[idx[i]] = top3[i].clone(); // Assign medals
            } else {
                ans[idx[i]] = (i + 1).to_string(); // Assign ranks
            }
        }

        ans
    }
}
