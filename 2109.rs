//2109. Adding Spaces to a String

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut ans = Vec::new();
        let mut j = 0;

        for (i, c) in s.chars().enumerate() {
            // Check if the current index `i` matches the space position in `spaces`
            if j < spaces.len() && i as i32 == spaces[j] {
                ans.push(' '); // Insert space
                j += 1; // Move to the next space position
            }
            ans.push(c); // Add the current character
        }

        // Convert the `Vec<char>` back to a `String`
        ans.into_iter().collect()
    }
}
