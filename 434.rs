//434. Number of Segments in a String

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_whitespace().count() as i32
    }
}
