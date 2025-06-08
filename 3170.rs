//3170. Lexicographically Minimum String After Removing Stars

impl Solution {
    pub fn clear_stars(s: String) -> String {
        if !s.contains('*') {
            return s;
        }

        let mut char_pos: Vec<Vec<usize>> = vec![vec![]; 26];
        let mut arr: Vec<char> = s.chars().collect();

        for i in 0..arr.len() {
            if arr[i] == '*' {
                for j in 0..26 {
                    if let Some(pos) = char_pos[j].pop() {
                        arr[pos] = '*';
                        break;
                    }
                }
            } else {
                let index = (arr[i] as u8 - b'a') as usize;
                char_pos[index].push(i);
            }
        }

        arr.into_iter().filter(|&c| c != '*').collect()
    }
}
