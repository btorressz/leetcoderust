//1910. Remove All Occurrences of a Substring

//Attempt 1 wrong answer
/*

impl Solution {
    pub fn remove_occurrences(mut s: String, part: String) -> String {
        // Keep removing 'part' from 's' as long as 'part' is a substring of 's'
        while s.contains(&part) {
            s = s.replace(&part, "");
        }
        s
    }
}


*/

//Attempt 2 successful
impl Solution {
    pub fn remove_occurrences(mut s: String, part: String) -> String {
        while let Some(pos) = s.find(&part) {
            s.replace_range(pos..pos + part.len(), "");
        }
        s
    }
}

