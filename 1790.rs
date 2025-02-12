//1790. Check if One String Swap Can Make Strings Equal

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let diff_indices: Vec<usize> = s1.chars()
            .zip(s2.chars())
            .enumerate()
            .filter_map(|(i, (c1, c2))| if c1 != c2 { Some(i) } else { None })
            .collect();

        match diff_indices.len() {
            0 => true,
            2 => {
                let (i, j) = (diff_indices[0], diff_indices[1]);
                let s1_chars: Vec<char> = s1.chars().collect();
                let s2_chars: Vec<char> = s2.chars().collect();
                s1_chars[i] == s2_chars[j] && s1_chars[j] == s2_chars[i]
            }
            _ => false,
        }
    }
}
